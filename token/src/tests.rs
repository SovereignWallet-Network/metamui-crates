use crate::mock::*;
use crate::types::*;
use crate::mock::{Did, Balances, Token, VC};
use super::*;
use frame_support::{assert_ok, assert_noop};
use sp_runtime::{app_crypto::{Pair, sr25519},  testing::H256, traits::{BlakeTwo256, Hash}};
use metamui_primitives::types::VC;

#[test]
fn test_slash_token() {
    new_test_ext().execute_with(|| {
        let currency_code: CurrencyCode = convert_to_array::<8>("OTH".into());
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
            owner: BOB,
            issuers: vec![BOB],
            is_vc_used: false,
            vc_property: token_vc,
        };

        assert_ok!(VC::store(
            Origin::signed(BOB_ACCOUNT_ID),
            vc_struct.encode()
        ));

        let vc_id = vc::Lookup::get(&BOB)[0];

        let token_amount: u128 = 5_000_000;
        // issue token
        assert_ok!(Token::issue_token(
            Origin::signed(BOB_ACCOUNT_ID),
            vc_id,
            token_amount
        ));

        let slash_amount: u128 = 1_000_000;
        let slash_vc = metamui_primitives::SlashMintTokens {
            vc_id,
            amount: slash_amount,
        };

        let slash_vc: [u8; 128] = convert_to_array::<128>(slash_vc.encode());
        let vc_type = vc::VCType::SlashTokens;
        let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
        let owner = DAVE;
        let issuers = vec![BOB];
        let hash = BlakeTwo256::hash_of(&(&vc_type, &slash_vc, &owner, &issuers));
        let signature = pair.sign(hash.as_ref());

        let vc_struct: VC<H256> = VC {
            hash,
            signatures: vec![signature],
            vc_type,
            owner,
            issuers,
            is_vc_used: false,
            vc_property: slash_vc,
            is_vc_active: todo!(),
        };

        assert_ok!(pallet_vc::store(
            Origin::signed(DAVE_ACCOUNT_ID),
            vc_struct.encode()
        ));
        let vc_id = pallet_vc::Lookup::get(&DAVE)[0];

        assert_ok!(Tokens::slash_token(Origin::signed(DAVE_ACCOUNT_ID), vc_id));

        // checking correctness of free balance after slash
        assert_eq!(
            Balances::free_balance(TEST_TOKEN_ID, &BOB_ACCOUNT_ID),
            token_amount - slash_amount
        );

        // checking slash token vc works after being used
        assert_noop!(
            Tokens::slash_token(Origin::signed(DAVE_ACCOUNT_ID), vc_id),
            vc::Error::<Test>::VCAlreadyUsed
        );
    });
}