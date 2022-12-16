use super::*;
use crate::mock::{VC,  *};
use sp_core::{sr25519, Pair, H256};
use metamui_primitives::types::{ TokenVC, VC as VCStruct};
use frame_support::{
	assert_noop, assert_ok,
};

#[test]
fn test_store() {
	new_test_ext().execute_with(|| {
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
		};

		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: true,
			is_vc_active: true,
			vc_type,
			vc_property: token_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()));
		let vc_id = *BlakeTwo256::hash_of(&vc).as_fixed_bytes();
		let did = RLookup::<Test>::get(vc_id);
		assert_eq!(did, BOB);
		assert_eq!(Lookup::<Test>::get(did), vec![vc_id]);
		assert_eq!(VCs::<Test>::get(vc_id), Some(vc.clone()));
		assert_eq!(VCHistory::<Test>::get(vc_id), Some((vc.is_vc_active, 0)));
	})
}

#[test]
fn test_invalid_owner_vc() {
	new_test_ext().execute_with(|| {
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let currency_code = convert_to_array::<8>("OTH".into());
		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code,
		};

		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: true,
			is_vc_active: true,
			vc_type,
			vc_property: token_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc).as_fixed_bytes();
		let did = RLookup::<Test>::get(vc_id);
		assert_eq!(did, BOB);
		assert_eq!(Lookup::<Test>::get(did), vec![vc_id]);
		assert_eq!(VCs::<Test>::get(vc_id), Some(vc.clone()));
		assert_eq!(VCHistory::<Test>::get(vc_id), Some((vc.is_vc_active, 0)));

		// Test MintVC
		let vc_type = VCType::MintTokens;
		let owner = ALICE;
		let issuers = vec![BOB];
		let mint_vc = SlashMintTokens { vc_id, currency_code, amount: 1000 };
		let mint_vc: [u8; 128] = convert_to_array::<128>(mint_vc.encode());
		let hash = BlakeTwo256::hash_of(&(&vc_type, &mint_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());
		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: true,
			is_vc_active: true,
			vc_type,
			vc_property: mint_vc,
		};
		// Since the owner Did (Dave) is not registered, this should fail
		assert_noop!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()),
		Error::<Test>::DidDoesNotExist
	);
	})
}

#[test]
fn test_mint_vc_store() {
	new_test_ext().execute_with(|| {
		let currency_code = convert_to_array::<8>("OTH".into());
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code,
		};

		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: true,
			is_vc_active: true,
			vc_type,
			vc_property: token_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc).as_fixed_bytes();
		let did = RLookup::<Test>::get(vc_id);
		assert_eq!(did, BOB);
		assert_eq!(Lookup::<Test>::get(did), vec![vc_id]);
		assert_eq!(VCs::<Test>::get(vc_id), Some(vc.clone()));
		assert_eq!(VCHistory::<Test>::get(vc_id), Some((vc.is_vc_active, 0)));

		let vc_type = VCType::MintTokens;
		let owner = DAVE;
		let issuers = vec![BOB];
		let mint_vc = SlashMintTokens { vc_id, currency_code, amount: 1000 };
		let mint_vc: [u8; 128] = convert_to_array::<128>(mint_vc.encode());
		let hash = BlakeTwo256::hash_of(&(&vc_type, &mint_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());
		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: false,
			is_vc_active: true,
			vc_type,
			vc_property: mint_vc,
		};
		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc).as_fixed_bytes();
		let did = RLookup::<Test>::get(vc_id);
		assert_eq!(did, DAVE);
		assert_eq!(Lookup::<Test>::get(did), vec![vc_id]);
		assert_eq!(VCs::<Test>::get(vc_id), Some(vc.clone()));
		assert_eq!(VCHistory::<Test>::get(vc_id), Some((vc.is_vc_active, 0)))
	})
}

#[test]
fn test_cccode_validation() {
	new_test_ext().execute_with(|| {
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTHs".into()),
		};

		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers: issuers.clone(),
			signatures: vec![signature.clone()],
			is_vc_used: true,
			is_vc_active: true,
			vc_type: vc_type.clone(),
			vc_property: token_vc,
		};

		assert_noop!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()),
		Error::<Test>::InvalidCurrencyCode
	);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>(" OT H".into()),
		};
		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			signatures: vec![signature.clone()],
			vc_type: vc_type.clone(),
			owner,
			issuers: issuers.clone(),
			is_vc_used: true,
			vc_property: token_vc,
			is_vc_active: true,
		};

		assert_noop!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()),
		Error::<Test>::InvalidCurrencyCode
	);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("1OTH".into()),
		};
		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: true,
			is_vc_active: true,
			vc_type,
			vc_property: token_vc,
		};

		assert_noop!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()),
		Error::<Test>::InvalidCurrencyCode
	);
	})
}

#[test]
fn test_update_status() {
	new_test_ext().execute_with(|| {
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
		};

		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: true,
			is_vc_active: true,
			vc_type,
			vc_property: token_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()));

		let vc_id = Lookup::<Test>::get(&BOB)[0];
		// Updating status flag
		assert_ok!(VC::update_status(Origin::signed(BOB_ACCOUNT_ID), vc_id, false));
 
		assert_eq!((VCs::<Test>::get(vc_id)).unwrap().is_vc_active, false);
	})
}

#[test]
fn test_store_vc_with_different_account() {
	new_test_ext().execute_with(|| {
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
		};

		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: true,
			is_vc_active: true,
			vc_type,
			vc_property: token_vc,
		};

		assert_noop!(VC::store(Origin::signed(NON_VALIDATOR_ACCOUNT), vc.encode()),
		DispatchError::BadOrigin
	);
	})
}

#[test]
fn test_store_vc_with_wrong_hash() {
	new_test_ext().execute_with(|| {
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
		};

		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		// Wrong Hash
		let hash = H256::zero();
		let signature = pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner: BOB,
			issuers: vec![BOB],
			signatures: vec![signature],
			is_vc_used: true,
			is_vc_active: true,
			vc_type,
			vc_property: token_vc,
		};

		assert_noop!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()),
		Error::<Test>::VCPropertiesNotVerified
	);
	})
}

#[test]
fn test_store_vc_with_wrong_signature() {
	new_test_ext().execute_with(|| {
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
		};

		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let wrong_hash = H256::zero();
		let signature = pair.sign(wrong_hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: true,
			is_vc_active: true,
			vc_type,
			vc_property: token_vc,
		};

		assert_noop!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()),
		Error::<Test>::InvalidSignature
	);
	})
}

#[test]
fn test_store_vc_less_approvers() {
	new_test_ext().execute_with(|| {
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
		};

		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB, DAVE];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let wrong_hash = H256::zero();
		let signature = pair.sign(wrong_hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: true,
			is_vc_active: true,
			vc_type,
			vc_property: token_vc,
		};

		assert_noop!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()),
		Error::<Test>::InvalidSignature
	);
	})
}

#[test]
fn test_update_status_sender() {
	new_test_ext().execute_with(|| {
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
		};

		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: true,
			is_vc_active: true,
			vc_type,
			vc_property: token_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()));

		let vc_id = Lookup::<Test>::get(&BOB)[0];
		let non_issuer = VALIDATOR_ACCOUNT;

		// Updating status flag with non issuer account
		assert_noop!(VC::update_status(Origin::signed(non_issuer), vc_id, vc.is_vc_active),
		Error::<Test>::NotAValidatorNorIssuer
	);

		// Updating status flag with non validator account
		assert_noop!(VC::update_status(Origin::signed(VALIDATOR_ACCOUNT), vc_id, vc.is_vc_active),
		Error::<Test>::NotAValidatorNorIssuer
	);
	})
}

#[test]
fn test_add_signature() {
	new_test_ext().execute_with(|| {
		let bob_pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let dave_pair: sr25519::Pair = sr25519::Pair::from_seed(&DAVE_SEED);
		let eve_pair: sr25519::Pair = sr25519::Pair::from_seed(&EVE_SEED);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
		};

		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB, DAVE, EVE];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let bob_sign = bob_pair.sign(hash.as_ref());
		let dave_sign = dave_pair.sign(hash.as_ref());
		let eve_sign = eve_pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![bob_sign.clone()],
			is_vc_used: true,
			is_vc_active: false,
			vc_type: vc_type.clone(),
			vc_property: token_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()));

		let vc_id = Lookup::<Test>::get(&BOB)[0];

		// vc_status = Inactive as only one issuer signed
		assert_eq!(VCs::<Test>::get(vc_id), Some(vc.clone()));

		// updating DAVE's signature
		let vc: VCStruct<H256> = VCStruct {
			hash,
			signatures: vec![bob_sign.clone(), dave_sign.clone()],
			vc_type: vc_type.clone(),
			owner: BOB,
			issuers: vec![BOB, DAVE, EVE],
			is_vc_used: true,
			vc_property: token_vc,
			is_vc_active: false,
		};

		assert_ok!(VC::add_signature(Origin::signed(BOB_ACCOUNT_ID), vc_id, dave_sign.clone()));

		// vc_status = Inactive as only two issuer signed
		assert_eq!(VCs::<Test>::get(vc_id), Some(vc.clone()));

		// updating EVE's signature
		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner: BOB,
			issuers: vec![BOB, DAVE, EVE],
			signatures: vec![bob_sign, dave_sign, eve_sign.clone()],
			is_vc_used: true,
			is_vc_active: true,
			vc_type,
			vc_property: token_vc,
		};

		assert_ok!(VC::add_signature(Origin::signed(BOB_ACCOUNT_ID), vc_id, eve_sign));

		// vc_status = Active as only all issuer signed
		assert_eq!(VCs::<Test>::get(vc_id), Some(vc.clone()));
	})
}

#[test]
fn test_add_signature_with_one_of_the_signers() {
	new_test_ext().execute_with(|| {
		let bob_pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let dave_pair: sr25519::Pair = sr25519::Pair::from_seed(&DAVE_SEED);
		let eve_pair: sr25519::Pair = sr25519::Pair::from_seed(&EVE_SEED);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
		};

		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB, DAVE, EVE];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let bob_sign = bob_pair.sign(hash.as_ref());
		// signed by Dave's public key
		let dave_sign = dave_pair.sign(hash.as_ref());
		// signed by Eve's public key
		let eve_sign = eve_pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![bob_sign.clone()],
			is_vc_used: true,
			is_vc_active: false,
			vc_type: vc_type.clone(),
			vc_property: token_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()));

		let vc_id = Lookup::<Test>::get(&BOB)[0];

		// vc_status = Inactive as only one issuer signed
		assert_eq!(VCs::<Test>::get(vc_id), Some(vc.clone()));

		// updating DAVE's signature
		let vc: VCStruct<H256> = VCStruct {
			hash,
			signatures: vec![bob_sign.clone(), dave_sign.clone()],
			vc_type: vc_type.clone(),
			owner: BOB,
			issuers: vec![BOB, DAVE, EVE],
			is_vc_used: true,
			vc_property: token_vc,
			is_vc_active: false,
		};

		assert_ok!(VC::add_signature(Origin::signed(DAVE_ACCOUNT_ID), vc_id, dave_sign.clone()));

		// vc_status = Inactive as only two issuer signed
		assert_eq!(VCs::<Test>::get(vc_id), Some(vc.clone()));

		// updating EVE's signature
		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner: BOB,
			issuers: vec![BOB, DAVE, EVE],
			signatures: vec![bob_sign, dave_sign, eve_sign.clone()],
			is_vc_used: true,
			is_vc_active: true,
			vc_type,
			vc_property: token_vc,
		};

		assert_ok!(VC::add_signature(Origin::signed(DAVE_ACCOUNT_ID), vc_id, eve_sign));

		// vc_status = Active as only all issuer signed
		assert_eq!(VCs::<Test>::get(vc_id), Some(vc.clone()));
	})
}

#[test]
fn test_set_is_used_flag() {
	new_test_ext().execute_with(|| {
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
		};

		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: false,
			is_vc_active: false,
			vc_type,
			vc_property: token_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()));

		let vc_id = Lookup::<Test>::get(&BOB)[0];

		// set vc is_used flag as true
		VC::set_is_used_flag(vc_id, Some(true));
		let vc_details = VCs::<Test>::get(vc_id).unwrap();
		assert!(vc_details.is_vc_used);
	})
}

#[test]
fn test_duplicate_issuers_signatures() {
	new_test_ext().execute_with(|| {
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
		};

		// case when duplicate signatures are present
		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature.clone(), signature.clone()],
			is_vc_used: true,
			is_vc_active: false,
			vc_type,
			vc_property: token_vc,
		};

		assert_noop!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()),
		Error::<Test>::DuplicateSignature);

		// case when duplicate issuers are present
		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB, BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: true,
			is_vc_active: false,
			vc_type,
			vc_property: token_vc,
		};

		assert_noop!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()),
		Error::<Test>::DuplicateSignature
	);
	})
}

#[test]
fn test_add_duplicate_issuer_signatures() {
	new_test_ext().execute_with(|| {
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);

		let dave_pair: sr25519::Pair = sr25519::Pair::from_seed(&DAVE_SEED);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
		};

		// case when duplicate signatures are present
		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());
		let duplicate_signature = pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers: issuers.clone(),
			signatures: vec![signature.clone(), duplicate_signature.clone()],
			is_vc_used: true,
			is_vc_active: false,
			vc_type: vc_type.clone(),
			vc_property: token_vc,
		};

		assert_noop!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()),
		Error::<Test>::DuplicateSignature
	);

		let dave_sign = dave_pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers: issuers.clone(),
			signatures: vec![signature.clone(), dave_sign],
			is_vc_used: true,
			is_vc_active: false,
			vc_type: vc_type.clone(),
			vc_property: token_vc,
		};

		assert_noop!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()),
		Error::<Test>::InvalidSignature
	);

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: true,
			is_vc_active: false,
			vc_type,
			vc_property: token_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()));

		let vc_id = Lookup::<Test>::get(&BOB)[0];

		assert_noop!(VC::add_signature(Origin::signed(DAVE_ACCOUNT_ID), vc_id, duplicate_signature),
		Error::<Test>::DuplicateSignature,
	);
	})
}

#[test]
fn test_generic_vc_store() {
	new_test_ext().execute_with(|| {
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);

		let generic_vc = GenericVC { cid: convert_to_array::<64>("F0TAeD_UY2mK-agbzZTW".into()) };

		let generic_vc: [u8; 128] = convert_to_array::<128>(generic_vc.encode());

		let vc_type = VCType::GenericVC;
		let owner = BOB;
		let issuers = vec![BOB];
		// Hash for generic vc will be generated using
		// the data stored in vc_url of generic_vc
		let hash = BlakeTwo256::hash_of(&generic_vc);
		let signature = pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![signature],
			is_vc_used: true,
			is_vc_active: true,
			vc_type,
			vc_property: generic_vc,
		};

		assert_noop!(
			VC::store(Origin::signed(VALIDATOR_ACCOUNT), vc.encode()),
			Error::<Test>::NotACouncilMember,
		);

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()));

		let vc_id = *BlakeTwo256::hash_of(&vc).as_fixed_bytes();
		let did = RLookup::<Test>::get(vc_id);
		assert_eq!(did, BOB);
		assert_eq!(Lookup::<Test>::get(did), vec![vc_id]);
		assert_eq!(VCs::<Test>::get(vc_id), Some(vc.clone()));
		assert_eq!(VCHistory::<Test>::get(vc_id), Some((vc.is_vc_active, 0)));
	})
}

#[test]
fn test_vc_already_exists() {
	new_test_ext().execute_with(|| {
		let pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
		};

		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![BOB];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let signature = pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers: issuers.clone(),
			signatures: vec![signature.clone()],
			is_vc_used: true,
			is_vc_active: false,
			vc_type: vc_type.clone(),
			vc_property: token_vc,
		};

		assert_ok!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()));

		assert_noop!(VC::store(Origin::signed(BOB_ACCOUNT_ID), vc.encode()), Error::<Test>::VCAlreadyExists);
	})
}

#[test]
fn test_invalid_signature_for_add_signature() {
	new_test_ext().execute_with(|| {
		let bob_pair: sr25519::Pair = sr25519::Pair::from_seed(&BOB_SEED);
		let dave_pair: sr25519::Pair = sr25519::Pair::from_seed(&DAVE_SEED);

		let token_vc = TokenVC {
			token_name: convert_to_array::<16>("test".into()),
			reservable_balance: 1000,
			decimal: 6,
			currency_code: convert_to_array::<8>("OTH".into()),
		};

		let token_vc: [u8; 128] = convert_to_array::<128>(token_vc.encode());
		let vc_type = VCType::TokenVC;
		let owner = BOB;
		let issuers = vec![DAVE];
		let hash = BlakeTwo256::hash_of(&(&vc_type, &token_vc, &owner, &issuers));
		let bob_sign = bob_pair.sign(hash.as_ref());
		let dave_sign = dave_pair.sign(hash.as_ref());

		let vc: VCStruct<H256> = VCStruct {
			hash,
			owner,
			issuers,
			signatures: vec![bob_sign.clone()],
			is_vc_used: true,
			is_vc_active: false,
			vc_type: vc_type.clone(),
			vc_property: token_vc,
		};

		let vc_id = *BlakeTwo256::hash_of(&vc).as_fixed_bytes();

		assert_ok!(VC::validate_sign(&vc, dave_sign.clone(), vc_id));
		//Error will occur If signed by someone who is not issuer, Signature will be invalid!
		assert_noop!(VC::validate_sign(&vc, bob_sign.clone(), vc_id), Error::<Test>::InvalidSignature);
	})
}
