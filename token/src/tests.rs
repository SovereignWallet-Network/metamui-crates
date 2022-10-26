use super::*;
use crate::{
	mock::{Balances, Token, VC, *},
	types::*,
};
use frame_support::assert_ok;
use metamui_primitives::types::{SlashMintTokens, VC as VCStruct};
use pallet_vc;
use sp_core::{sr25519, Pair, H256};
use sp_runtime::traits::{BlakeTwo256, Hash};

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
		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		let token_amount: u128 = 5_000_000;

		let _ = Balances::deposit_creating(&BOB_ACCOUNT_ID, token_amount.try_into().unwrap());

		let mint_amount: u128 = 1_000_000;
		let mint_vc = SlashMintTokens { vc_id, amount: mint_amount };

		let mint_vc: [u8; 128] = convert_to_array::<128>(mint_vc.encode());
		let vc_type = VCType::MintTokens;
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let owner = BOB;
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

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));
		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		assert_ok!(Token::mint_token(Origin::signed(BOB_ACCOUNT_ID), vc_id));

		// checking correctness of free balance after mint
		assert_eq!(Balances::free_balance(&BOB_ACCOUNT_ID), (token_amount + mint_amount) as u64);
		assert_eq!(Balances::total_issuance(), (token_amount + mint_amount) as u64);
	});
}

#[test]
#[should_panic]
fn test_mint_token_fails() {
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
		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		let token_amount: u128 = 5_000_000;

		let _ = Balances::deposit_creating(&BOB_ACCOUNT_ID, token_amount.try_into().unwrap());

		let mint_amount: u128 = 1_000_000;
		let mint_vc = SlashMintTokens { vc_id, amount: mint_amount };

		let mint_vc: [u8; 128] = convert_to_array::<128>(mint_vc.encode());
		let vc_type = VCType::MintTokens;
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let owner = BOB;
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

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));
		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		assert_ok!(Token::mint_token(Origin::signed(BOB_ACCOUNT_ID), vc_id));

		// checking correctness of free balance after mint
		assert_eq!(Balances::free_balance(&BOB_ACCOUNT_ID), (token_amount + mint_amount) as u64);
		assert_eq!(Balances::total_issuance(), (token_amount + mint_amount) as u64);

		// checking mint token vc works after being used
		assert_ok!(Token::mint_token(Origin::signed(BOB_ACCOUNT_ID), vc_id));
	});
}


#[test]
#[should_panic]
fn test_mint_token_fails_invalidvc() {
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
		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		let token_amount: u128 = 5_000_000;

		let _ = Balances::deposit_creating(&BOB_ACCOUNT_ID, token_amount.try_into().unwrap());

		let mint_amount: u128 = 1_000_000;
		let mint_vc = SlashMintTokens { vc_id, amount: mint_amount };

		let mint_vc: [u8; 128] = convert_to_array::<128>(mint_vc.encode());
		let vc_type = VCType::SlashTokens;
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let owner = BOB;
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

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));
		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		assert_ok!(Token::mint_token(Origin::signed(BOB_ACCOUNT_ID), vc_id));

	});
}

#[test]
fn test_slash_token() {
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
		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		let token_amount: u128 = 5_000_000;

		let _ = Balances::deposit_creating(&BOB_ACCOUNT_ID, token_amount.try_into().unwrap());

		let slash_amount: u128 = 1_000_000;
		let slash_vc = SlashMintTokens { vc_id, amount: slash_amount };

		let slash_vc: [u8; 128] = convert_to_array::<128>(slash_vc.encode());
		let vc_type = VCType::SlashTokens;
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &slash_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc_struct: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: false,
			is_vc_active: false,
			vc_type,
			vc_property: slash_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));
		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		assert_ok!(Token::slash_token(Origin::signed(BOB_ACCOUNT_ID), vc_id));

		// checking correctness of free balance after mint
		assert_eq!(Balances::free_balance(&BOB_ACCOUNT_ID), (token_amount - slash_amount) as u64);
		assert_eq!(Balances::total_issuance(), (token_amount - slash_amount) as u64);
	});
}

#[test]
#[should_panic]
fn test_slash_token_fails() {
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
		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		let token_amount: u128 = 5_000_000;

		let _ = Balances::deposit_creating(&BOB_ACCOUNT_ID, token_amount.try_into().unwrap());

		let slash_amount: u128 = 1_000_000;
		let slash_vc = SlashMintTokens { vc_id, amount: slash_amount };

		let slash_vc: [u8; 128] = convert_to_array::<128>(slash_vc.encode());
		let vc_type = VCType::SlashTokens;
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &slash_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc_struct: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: false,
			is_vc_active: false,
			vc_type,
			vc_property: slash_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));
		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		assert_ok!(Token::slash_token(Origin::signed(BOB_ACCOUNT_ID), vc_id));

		// checking correctness of free balance after mint
		assert_eq!(Balances::free_balance(&BOB_ACCOUNT_ID), (token_amount - slash_amount) as u64);
		assert_eq!(Balances::total_issuance(), (token_amount - slash_amount) as u64);

		// checking slash token vc works after being used
		assert_ok!(Token::slash_token(Origin::signed(BOB_ACCOUNT_ID), vc_id));
	});
}

#[test]
#[should_panic]
fn test_slash_token_fails_invalidvc() {
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
		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		let token_amount: u128 = 5_000_000;

		let _ = Balances::deposit_creating(&BOB_ACCOUNT_ID, token_amount.try_into().unwrap());

		let slash_amount: u128 = 1_000_000;
		let slash_vc = SlashMintTokens { vc_id, amount: slash_amount };

		let slash_vc: [u8; 128] = convert_to_array::<128>(slash_vc.encode());
		let vc_type = VCType::MintTokens;
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &slash_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc_struct: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: false,
			is_vc_active: false,
			vc_type,
			vc_property: slash_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));
		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		assert_ok!(Token::slash_token(Origin::signed(BOB_ACCOUNT_ID), vc_id));
	});
}

#[test]
#[should_panic]
fn test_slash_token_fails_lowbalance() {
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
		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		let token_amount: u128 = 2_000_000;

		let _ = Balances::deposit_creating(&BOB_ACCOUNT_ID, token_amount.try_into().unwrap());

		let slash_amount: u128 = 3_000_000;
		let slash_vc = SlashMintTokens { vc_id, amount: slash_amount };

		let slash_vc: [u8; 128] = convert_to_array::<128>(slash_vc.encode());
		let vc_type = VCType::SlashTokens;
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &slash_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc_struct: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: false,
			is_vc_active: false,
			vc_type,
			vc_property: slash_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));
		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		assert_ok!(Token::slash_token(Origin::signed(BOB_ACCOUNT_ID), vc_id));
	});
}


#[test]
fn test_withdraw_reserve_works() {
	new_test_ext().execute_with(|| {
		let reservable_balance: u128 = 1_000_000;
		let token_amount: u128 = 5_000_000;
		let currency_code: CurrencyCode = convert_to_array::<8>("OTH".into());
		let token_vc = pallet_vc::TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
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

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		let _ = Balances::deposit_creating(&BOB_ACCOUNT_ID, token_amount.try_into().unwrap());

		assert_ok!(Token::withdraw_reserved(Origin::signed(BOB_ACCOUNT_ID), DAVE, BOB, 1_000_000));

		// check balance has been credited correctly
		assert_eq!(
			Balances::free_balance(BOB_ACCOUNT_ID),
			(token_amount - reservable_balance) as u64
		);
		assert_eq!(
			Balances::reserved_balance(BOB_ACCOUNT_ID),
			(reservable_balance - 1_000_000) as u64
		);
		assert_eq!(Balances::total_balance(&BOB_ACCOUNT_ID), (token_amount - 1_000_000) as u64);
		assert_eq!(Balances::free_balance(DAVE_ACCOUNT_ID), (INITIAL_BALANCE + 1_000_000) as u64);

		// check created token details
		assert_eq!(Balances::total_issuance(), 5000000);
	});
}

#[test]
#[should_panic]
fn test_withdraw_reserve_fails() {
	new_test_ext().execute_with(|| {
		let reservable_balance: u128 = 1_000_000;
		let token_amount: u128 = 5_000_000;
		let currency_code: CurrencyCode = convert_to_array::<8>("OTH".into());
		let token_vc = pallet_vc::TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
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

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		let _ = Balances::deposit_creating(&BOB_ACCOUNT_ID, token_amount.try_into().unwrap());

		assert_ok!(Token::withdraw_reserved(Origin::signed(BOB_ACCOUNT_ID), ALICE, BOB, 1_000_000));
	});
}

#[test]
fn test_transfer_token_works() {
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
		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		let token_amount: u128 = 5_000_000;

		let _ = Balances::deposit_creating(&BOB_ACCOUNT_ID, token_amount.try_into().unwrap());

		let transfer_amount: u128 = 1_000_000;
		let token_transfer_vc = TokenTransferVC { vc_id, amount: transfer_amount };

		let token_transfer_vc: [u8; 128] = convert_to_array::<128>(token_transfer_vc.encode());
		let vc_type = VCType::TokenTransferVC;
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_transfer_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc_struct: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: false,
			is_vc_active: false,
			vc_type,
			vc_property: token_transfer_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));
		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();
		
		assert_ok!(Token::transfer_token(Origin::signed(BOB_ACCOUNT_ID), vc_id, DAVE));

		// check balance transfer worked correctly
		assert_eq!(Balances::free_balance(BOB_ACCOUNT_ID), (token_amount - transfer_amount) as u64);
		assert_eq!(Balances::free_balance(DAVE_ACCOUNT_ID), transfer_amount as u64);
		assert_eq!(Balances::total_issuance(), token_amount as u64);
	});
}


#[test]
#[should_panic]
fn test_transfer_token_fails_lowbalance() {
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
		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		let token_amount: u128 = 5_000_000;

		let _ = Balances::deposit_creating(&BOB_ACCOUNT_ID, token_amount.try_into().unwrap());

		let transfer_amount: u128 = 7_000_000;
		let token_transfer_vc = TokenTransferVC { vc_id, amount: transfer_amount };

		let token_transfer_vc: [u8; 128] = convert_to_array::<128>(token_transfer_vc.encode());
		let vc_type = VCType::TokenTransferVC;
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_transfer_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc_struct: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: false,
			is_vc_active: false,
			vc_type,
			vc_property: token_transfer_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));
		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();
		
		assert_ok!(Token::transfer_token(Origin::signed(BOB_ACCOUNT_ID), vc_id, DAVE));
	});
}


#[test]
#[should_panic]
fn test_transfer_token_fails_invalidvc() {
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
		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();

		let token_amount: u128 = 5_000_000;

		let _ = Balances::deposit_creating(&BOB_ACCOUNT_ID, token_amount.try_into().unwrap());

		let transfer_amount: u128 = 1_000_000;
		let token_transfer_vc = TokenTransferVC { vc_id, amount: transfer_amount };

		let token_transfer_vc: [u8; 128] = convert_to_array::<128>(token_transfer_vc.encode());
		let vc_type = VCType::SlashTokens;
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_transfer_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc_struct: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: false,
			is_vc_active: false,
			vc_type,
			vc_property: token_transfer_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc_struct.encode()));
		let vc_id = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();
		
		assert_ok!(Token::transfer_token(Origin::signed(BOB_ACCOUNT_ID), vc_id, DAVE));
	});
}