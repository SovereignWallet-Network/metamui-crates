use crate::mock::*;
use crate::types::*;
use crate::mock::Did;
use super::*;

// use frame_support::{
// 	assert_noop, assert_ok, parameter_types,
// };
// use frame_system as system;
// use sp_core::{sr25519, H256};
// use sp_runtime::{testing::Header, traits::BlakeTwo256, BuildStorage};
// use validator_set;



#[test]
fn test_genesis_worked() {
	new_test_ext().execute_with(|| {
		assert_eq!(DIDs::<Test>::contains_key(VALIDATOR_DID.clone()), true);
		assert_eq!(Lookup::<Test>::contains_key(VALIDATOR_DID.clone()), true);
		assert_eq!(
			RLookup::<Test>::contains_key(Did::get_accountid_from_pubkey(&VALIDATOR_PUBKEY)),
			true
		);

		let (did_doc, block_number) = Did::get_did_details(VALIDATOR_DID.clone()).unwrap();
		match did_doc {
			DIDType::Public(public_did) => {
				assert_eq!(public_did.identifier, VALIDATOR_DID);
		    assert_eq!(public_did.public_key, VALIDATOR_PUBKEY);
			},
			DIDType::Private(private_did) => {
				assert_eq!(private_did.identifier, VALIDATOR_DID);
		    assert_eq!(private_did.public_key, VALIDATOR_PUBKEY);
			},
		}
		assert_eq!(block_number, 0);
	})
}

// // START ADD_DID TESTING
// #[test]
// #[should_panic]
// fn test_add_invalid_did() {
// 	new_test_ext().execute_with(|| {
// 		let identifier = *b"d\0d:ssid:Alice2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		let public_key = sr25519::Public([0; 32]);
// 		let metadata = vec![];

// 		assert_ok!(Did::add(Origin::signed(VALIDATOR_ACCOUNT), public_key, identifier, metadata));
// 	})
// }

// #[test]
// #[should_panic]
// fn test_non_validator_adds_did() {
// 	new_test_ext().execute_with(|| {
// 		let identifier = *b"Alice2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		let public_key = sr25519::Public([0; 32]);
// 		let metadata = vec![];

// 		assert_ok!(Did::add(
// 			Origin::signed(NON_VALIDATOR_ACCOUNT),
// 			public_key,
// 			identifier,
// 			metadata
// 		));
// 	})
// }

// #[test]
// fn test_add_did() {
// 	new_test_ext().execute_with(|| {
// 		let identifier = *b"did:ssid:Bob\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		let public_key = sr25519::Public([2; 32]);
// 		let metadata = "metadata".as_bytes().to_vec();

// 		assert_ok!(Did::add(
// 			Origin::signed(VALIDATOR_ACCOUNT),
// 			public_key,
// 			identifier,
// 			metadata.clone()
// 		));

// 		assert_eq!(DIDs::<Test>::contains_key(identifier.clone()), true);
// 		assert_eq!(Lookup::<Test>::contains_key(identifier.clone()), true);
// 		assert_eq!(
// 			RLookup::<Test>::contains_key(Did::get_accountid_from_pubkey(&public_key)),
// 			true
// 		);

// 		let (did_doc, _block_number) = Did::get_did_details(identifier.clone()).unwrap();
// 		assert_eq!(did_doc.identifier, identifier);
// 		assert_eq!(did_doc.public_key, public_key);
// 		assert_eq!(did_doc.metadata, metadata);

// 		let did_lookup = RLookup::<Test>::get(Did::get_accountid_from_pubkey(&public_key));
// 		assert_eq!(did_lookup, identifier.clone());
// 	})
// }

// #[test]
// #[should_panic]
// fn test_add_existing_did() {
// 	new_test_ext().execute_with(|| {
// 		// Adding the DID initialised at the time of genesis, so this test should fail
// 		let identifier = *b"did:ssid:Alice\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		let public_key = sr25519::Public([2; 32]);
// 		let metadata = vec![];

// 		assert_ok!(Did::add(
// 			Origin::signed(VALIDATOR_ACCOUNT),
// 			public_key,
// 			identifier,
// 			metadata.clone()
// 		));
// 	})
// }

// //END ADD_DID TESTING

// //START ADD_PUBLIC_KEY TESTING
// #[test]
// #[should_panic]
// fn test_add_existing_pubkey() {
// 	new_test_ext().execute_with(|| {
// 		let identifier = *b"did:ssid:Alicx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		let public_key = sr25519::Public([3; 32]);
// 		let metadata = vec![];

// 		assert_ok!(Did::add(
// 			Origin::signed(VALIDATOR_ACCOUNT),
// 			public_key,
// 			identifier,
// 			metadata.clone()
// 		));

// 		let identifier = *b"did:ssid:Alicx2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		let public_key = sr25519::Public([3; 32]);
// 		let metadata = vec![];

// 		assert_ok!(Did::add(
// 			Origin::signed(VALIDATOR_ACCOUNT),
// 			public_key,
// 			identifier,
// 			metadata.clone()
// 		));
// 	})
// }
// //END ADD_PUBLIC_KEY TESTING

// #[test]
// #[should_panic]
// fn test_remove_non_existing_did() {
// 	new_test_ext().execute_with(|| {
// 		let identifier = *b"did:ssid:DoesNotExist\0\0\0\0\0\0\0\0\0\0\0";

// 		assert_ok!(Did::remove(Origin::signed(VALIDATOR_ACCOUNT), identifier.clone()));
// 	})
// }

// //START REMOVE_DID TESTING
// #[test]
// #[should_panic]
// fn test_non_validator_removes_did() {
// 	new_test_ext().execute_with(|| {
// 		let identifier = *b"did:ssid:swn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

// 		assert_ok!(Did::remove(Origin::signed(NON_VALIDATOR_ACCOUNT), identifier.clone()));
// 	})
// }

// #[test]
// fn test_remove_did() {
// 	new_test_ext().execute_with(|| {
// 		let identifier = *b"did:ssid:Alicx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		let public_key = sr25519::Public([3; 32]);
// 		let metadata = vec![];

// 		assert_ok!(Did::add(
// 			Origin::signed(VALIDATOR_ACCOUNT),
// 			public_key,
// 			identifier,
// 			metadata.clone()
// 		));

// 		assert_ok!(Did::remove(Origin::signed(VALIDATOR_ACCOUNT), identifier.clone()));

// 		assert_eq!(DIDs::<Test>::contains_key(identifier.clone()), false);
// 		assert_eq!(Lookup::<Test>::contains_key(identifier.clone()), false);
// 		assert_eq!(
// 			RLookup::<Test>::contains_key(Did::get_accountid_from_pubkey(&public_key)),
// 			false
// 		);
// 	})
// }
// //END REMOVE_DID TESTING

// //START ROTATE_KEY TESTING
// #[test]
// fn test_rotate_key() {
// 	new_test_ext().execute_with(|| {
// 		let identifier = *b"did:ssid:Alicx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		let public_key = sr25519::Public([3; 32]);
// 		let metadata = vec![];

// 		assert_ok!(Did::add(
// 			Origin::signed(VALIDATOR_ACCOUNT),
// 			public_key,
// 			identifier.clone(),
// 			metadata.clone()
// 		));

// 		let public_key2 = sr25519::Public([4; 32]);

// 		run_to_block(3);

// 		assert_ok!(Did::rotate_key(
// 			Origin::signed(VALIDATOR_ACCOUNT),
// 			identifier.clone(),
// 			public_key2
// 		));

// 		assert_eq!(DIDs::<Test>::contains_key(identifier.clone()), true);
// 		assert_eq!(Lookup::<Test>::contains_key(identifier.clone()), true);

// 		// Ensure only a singly pubkey is mapped to a DID - inspired from toufeeq's testing
// 		assert_eq!(
// 			RLookup::<Test>::contains_key(Did::get_accountid_from_pubkey(&public_key)),
// 			false
// 		);
// 		assert_eq!(
// 			RLookup::<Test>::contains_key(Did::get_accountid_from_pubkey(&public_key2)),
// 			true
// 		);

// 		let (did_doc, block_number) = Did::get_did_details(identifier.clone()).unwrap();
// 		assert_eq!(did_doc.identifier, identifier);
// 		assert_eq!(did_doc.public_key, public_key2);
// 		assert_eq!(did_doc.metadata, metadata);
// 		assert_eq!(block_number, 3);

// 		// check the rotated key has been added to the history of the DID
// 		assert_eq!(PrevKeys::<Test>::contains_key(identifier.clone()), true);
// 		let prev_key_list = Did::get_prev_key_details(identifier.clone()).unwrap();
// 		assert_eq!(prev_key_list.is_empty(), false);
// 		assert_eq!(prev_key_list.len(), 1);

// 		let (last_pub_key, block_number) = prev_key_list.first().cloned().unwrap();
// 		assert_eq!(last_pub_key, Did::get_accountid_from_pubkey(&public_key));
// 		assert_eq!(block_number, 0);
// 	})
// }

// #[test]
// fn test_rotate_key_history() {
// 	new_test_ext().execute_with(|| {
// 		let identifier = *b"did:ssid:Alicx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		let public_key = sr25519::Public([3; 32]);
// 		let metadata = vec![];

// 		assert_ok!(Did::add(
// 			Origin::signed(VALIDATOR_ACCOUNT),
// 			public_key,
// 			identifier,
// 			metadata.clone()
// 		));

// 		let public_key2 = sr25519::Public([4; 32]);

// 		run_to_block(3);

// 		assert_ok!(Did::rotate_key(
// 			Origin::signed(VALIDATOR_ACCOUNT),
// 			identifier.clone(),
// 			public_key2
// 		));

// 		run_to_block(8);

// 		let public_key3 = sr25519::Public([7; 32]);

// 		assert_ok!(Did::rotate_key(
// 			Origin::signed(VALIDATOR_ACCOUNT),
// 			identifier.clone(),
// 			public_key3
// 		));

// 		assert_eq!(DIDs::<Test>::contains_key(identifier.clone()), true);
// 		assert_eq!(Lookup::<Test>::contains_key(identifier.clone()), true);

// 		// Ensure only a singly pubkey is mapped to a DID -  inspired from toufeeq's testing
// 		assert_eq!(
// 			RLookup::<Test>::contains_key(Did::get_accountid_from_pubkey(&public_key)),
// 			false
// 		);
// 		assert_eq!(
// 			RLookup::<Test>::contains_key(Did::get_accountid_from_pubkey(&public_key2)),
// 			false
// 		);
// 		assert_eq!(
// 			RLookup::<Test>::contains_key(Did::get_accountid_from_pubkey(&public_key3)),
// 			true
// 		);

// 		let (did_doc, block_number) = Did::get_did_details(identifier.clone()).unwrap();
// 		assert_eq!(did_doc.identifier, identifier);
// 		assert_eq!(did_doc.public_key, public_key3);
// 		assert_eq!(did_doc.metadata, metadata);
// 		assert_eq!(block_number, 8);

// 		// check the rotated key has been added to the history of the DID
// 		assert_eq!(PrevKeys::<Test>::contains_key(identifier.clone()), true);
// 		let prev_key_list = Did::get_prev_key_details(identifier.clone()).unwrap();
// 		assert_eq!(prev_key_list.is_empty(), false);
// 		assert_eq!(prev_key_list.len(), 2);

// 		let (last_pub_key, block_number) = prev_key_list[0];
// 		assert_eq!(last_pub_key, Did::get_accountid_from_pubkey(&public_key));
// 		assert_eq!(block_number, 0);

// 		let (last_pub_key2, block_number2) = prev_key_list[1];
// 		assert_eq!(last_pub_key2, Did::get_accountid_from_pubkey(&public_key2));
// 		assert_eq!(block_number2, 3);
// 	})
// }

// //END ROTATE_KEY TESTING

// //START ROTATE_DID TESTING
// #[test]
// #[should_panic]
// fn test_rotate_did_for_non_existent_did() {
// 	new_test_ext().execute_with(|| {
// 		let identifier = *b"did:ssid:Alicx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		let public_key = sr25519::Public([5; 32]);
// 		let metadata = vec![];

// 		assert_ok!(Did::add(
// 			Origin::signed(VALIDATOR_ACCOUNT),
// 			public_key,
// 			identifier,
// 			metadata.clone()
// 		));

// 		let identifier2 = *b"Alice2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

// 		assert_ok!(Did::rotate_key(
// 			Origin::signed(VALIDATOR_ACCOUNT),
// 			identifier2.clone(),
// 			public_key
// 		));
// 	})
// }
// //END ROTATE_DID TESTING

// //START UPADATE_METADATA TESTING
// #[test]
// fn test_metadata_updation() {
// 	new_test_ext().execute_with(|| {
// 		let identifier = *b"did:ssid:Alicx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		let public_key = sr25519::Public([5; 32]);
// 		let old_metadata = vec![];

// 		//add a new did
// 		assert_ok!(Did::add(
// 			Origin::signed(VALIDATOR_ACCOUNT),
// 			public_key,
// 			identifier,
// 			old_metadata.clone()
// 		));

// 		//assign new metadata to a variable
// 		let new_metadata: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0];

// 		//update the existing metadata with new metadata
// 		assert_ok!(Did::update_metadata(
// 			Origin::signed(VALIDATOR_ACCOUNT),
// 			identifier,
// 			new_metadata.clone()
// 		));

// 		//fetch did details
// 		let (did_doc, _block_number) = Did::get_did_details(identifier.clone()).unwrap();

// 		//check if the details are same as the ones we added above
// 		assert_eq!(did_doc.identifier, identifier);
// 		assert_eq!(did_doc.public_key, public_key);

// 		//check if the current metadata is not the same as the old metadata
// 		assert_ne!(did_doc.metadata, old_metadata);

// 		//check if the current metadata is the same as the new metadata
// 		assert_eq!(did_doc.metadata, new_metadata);
// 	})
// }
// //END UPADATE_METADATA TESTING

// //TESTING FUNCTIONS DEFINED IN THE MODULE-IMPL
// //START VALIDATE_DID TESTING
// #[test]
// fn test_did_validation() {
// 	new_test_ext().execute_with(|| {
// 		// without did: prefix
// 		let without_did_colon = *b"Alicx\0\0\0\0\0\0\0\0\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		assert!(!Did::is_did_valid(without_did_colon));

// 		// zero did
// 		let zero_did = [0; 32];
// 		assert!(!Did::is_did_valid(zero_did));

// 		// zero after did: prefix
// 		let zero_after_did_colon = *b"did:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		assert!(!Did::is_did_valid(zero_after_did_colon));

// 		// space followed by zeros
// 		let space_followed_by_zero =
// 			*b" \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		assert!(!Did::is_did_valid(space_followed_by_zero));

// 		// space followed by correct did
// 		let space_followed_correct_did = *b" did:ssid:swn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		assert!(!Did::is_did_valid(space_followed_correct_did));

// 		// correct did
// 		let correct_did = *b"did:ssid:swn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		assert!(Did::is_did_valid(correct_did));
// 	})
// }
// //END VALIDATE_DID TESTING

// //START GET_DID_DETAILS TESTING
// #[test]
// fn test_get_did_details() {
// 	new_test_ext().execute_with(|| {
// 		let identifier = *b"did:ssid:Alicx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// 		let public_key = sr25519::Public([5; 32]);
// 		let metadata = vec![];

// 		//add a new did
// 		assert_ok!(Did::add(
// 			Origin::signed(VALIDATOR_ACCOUNT),
// 			public_key,
// 			identifier,
// 			metadata.clone()
// 		));

// 		let (did_doc, _block_number) = Did::get_did_details(identifier.clone()).unwrap();
// 		assert_eq!(did_doc.identifier, identifier);
// 		assert_eq!(did_doc.public_key, public_key);
// 		assert_eq!(did_doc.metadata, metadata);
// 	})
// }
// //END GET_DID_DETAILS TESTING
