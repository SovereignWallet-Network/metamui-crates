use crate::mock::*;
use crate::types::*;
use pallet_vc;
use crate::mock::{Did, Balances, Token};
use super::*;
use frame_support::{assert_ok};
use sp_runtime::{app_crypto::{Pair, sr25519},  testing::H256, traits::{BlakeTwo256, Hash}};
use metamui_primitives::types::{VC as OtherVC, SlashMintTokens, VCType};

#[test]
#[should_panic]
fn test_slash_token() {
    new_test_ext().execute_with(|| {

        let vc_id = pallet_vc::Lookup::<Test>::get(&BOB)[0];
        let token_amount: u128 = 5_000_000;
        let slash_amount: u128 = 1_000_000;

        assert_ok!(Token::slash_token(Origin::signed(DAVE_ACCOUNT_ID), vc_id));
        // checking correctness of free balance after slash
        assert_eq!(
            Balances::free_balance(&BOB_ACCOUNT_ID),
            (token_amount - slash_amount) as u64
        );
        // checking slash token vc works after being used
        assert_ok!(
            Token::slash_token(Origin::signed(DAVE_ACCOUNT_ID), vc_id)
        );
    });
}