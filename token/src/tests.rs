use crate::mock::*;
use pallet_vc;
use crate::mock::{VC, Token, Balances};
use crate::types::*;
use sp_core::{Pair, sr25519, H256};
use sp_runtime::traits::{BlakeTwo256, Hash};
use super::*;
use frame_support::{assert_ok};
use metamui_primitives::types::{VC as VCStruct, SlashMintTokens};

#[test]
fn test_mint_token() {
	new_test_ext().execute_with(|| {
        let currency_code: CurrencyCode = convert_to_array::<8>("OTH".into());
        let token_vc = pallet_vc::TokenVC {
            token_name: convert_to_array::<16>("test".into()),
            reservable_balance: 1000,
            decimal: 6,
            currency_code,
        };

        let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
        let vc_type = VCType::TokenVC;
        let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
        let owner = BOB;
        let issuers = vec![BOB];
        let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
        let signature = pair.sign(hash.as_ref());

        let vc_struct: VCStruct<H256> = VCStruct {
            hash,
            owner,
            issuers,
            signatures: vec![signature],
            is_vc_used: false,
            is_vc_active: false,
            vc_type,
            vc_property: token_vc,
        };

        assert_ok!(VC::store(
            Origin::signed(BOB_ACCOUNT_ID),
            vc_struct.encode()
        ));

        let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

        let token_amount: u128 = 5_000_000;

        let _ = Balances::deposit_creating(&BOB_ACCOUNT_ID, token_amount.try_into().unwrap());

        let mint_amount: u128 = 1_000_000;
        let mint_vc = SlashMintTokens {
            vc_id,
            amount: mint_amount,
        };

        let mint_vc: [u8; 128] = convert_to_array::<128>(mint_vc.encode());
        let vc_type = VCType::MintTokens;
        let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
        let owner = DAVE;
        let issuers = vec![BOB];
        let hash = BlakeTwo256::hash_of(&(&vc_type, &mint_vc, &owner, &issuers));
        let signature = pair.sign(hash.as_ref());

        let vc_struct: VCStruct<H256> = VCStruct {
            hash,
            owner,
            issuers,
            signatures: vec![signature],
            is_vc_used: false,
            is_vc_active: false,
            vc_type,
            vc_property: mint_vc,
        };

        assert_ok!(VC::store(
            Origin::signed(DAVE_ACCOUNT_ID),
            vc_struct.encode()
        ));
        let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

        assert_ok!(Token::mint_token(Origin::signed(DAVE_ACCOUNT_ID), vc_id));

        // checking correctness of free balance after mint
        assert_eq!(
            Balances::free_balance(&BOB_ACCOUNT_ID),
            (token_amount + mint_amount) as u64
        );
        assert_eq!(
            Balances::total_issuance(),
            (token_amount + mint_amount) as u64
        );

        // checking mint token vc works after being used
        assert_ok!(
            Token::mint_token(Origin::signed(DAVE_ACCOUNT_ID), vc_id)
        );
    });
}