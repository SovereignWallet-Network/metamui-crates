use crate::mock::*;
use crate::types::*;
use pallet_vc;
use sp_core::H256;
use sp_core::Pair;
use sp_core::sr25519;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::traits::Hash;
use crate::mock::{Balances, Token};
use super::*;
use frame_support::{assert_ok};
use metamui_primitives::types::VC;

#[test]
fn test_mint_token() {
	new_test_ext().execute_with(|| {
        let token_vc = vc::TokenVC {
            token_name: convert_to_array::<16>("test".into()),
            reservable_balance: 1000,
            decimal: 6,
            currency_code,
        };

        let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
        let vc_type = vc::VCType::TokenVC;
        let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
        let owner = BOB;
        let issuers = vec![BOB];
        let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
        let signature = pair.sign(hash.as_ref());

        let vc_struct: vc::VC<H256> = vc::VC {
            hash,
            signatures: vec![signature],
            vc_type,
            owner,
            issuers,
            is_vc_used: false,
            vc_property: token_vc,
        };

        assert_ok!(VC::store(
            Origin::signed(BOB_ACCOUNT_ID),
            vc_struct.encode()
        ));

        let vc_id = vc::Lookup::get(&BOB)[0];

        let token_amount: u128 = 5_000_000;

        let mint_amount: u128 = 1_000_000;
        let mint_vc = vc::SlashMintTokens {
            vc_id,
            currency_code,
            amount: mint_amount,
        };

        let mint_vc: [u8; 128] = convert_to_array::<128>(mint_vc.encode());
        let vc_type = vc::VCType::MintTokens;
        let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
        let owner = DAVE;
        let issuers = vec![BOB];
        let hash = BlakeTwo256::hash_of(&(&vc_type, &mint_vc, &owner, &issuers));
        let signature = pair.sign(hash.as_ref());

        let vc_struct: vc::VC<H256> = vc::VC {
            hash,
            signatures: vec![signature],
            vc_type,
            owner,
            issuers,
            is_vc_used: false,
            vc_property: mint_vc,
        };

        assert_ok!(VC::store(
            Origin::signed(DAVE_ACCOUNT_ID),
            vc_struct.encode()
        ));
        let vc_id = vc::Lookup::get(&DAVE)[0];

        assert_ok!(Tokens::mint_token(Origin::signed(DAVE_ACCOUNT_ID), vc_id));

        // checking correctness of free balance after mint
        assert_eq!(
            Tokens::free_balance(TEST_TOKEN_ID, &BOB_ACCOUNT_ID),
            token_amount + mint_amount
        );
        assert_eq!(
            Tokens::total_issuance(currency_code),
            token_amount + mint_amount
        );

        // checking mint token vc works after being used
        assert_noop!(
            Tokens::mint_token(Origin::signed(DAVE_ACCOUNT_ID), vc_id),
            vc::Error::<Test>::VCAlreadyUsed
        );
    });
}