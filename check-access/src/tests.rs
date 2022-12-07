use crate::mock::*;
use super::*;
use frame_support::{ assert_ok, assert_noop };

#[test]
fn test_genesis_worked() {
	new_test_ext().execute_with(|| {
		// check WhitelistedPallets storage
		assert_eq!(WhitelistedPallets::<Test>::contains_key(FIRST_PALLET_NAME, FIRST_FUNCTION_NAME), true);
		let empty_tuple = WhitelistedPallets::<Test>::get(FIRST_PALLET_NAME, FIRST_FUNCTION_NAME);
		assert_eq!(empty_tuple == (), true);

		// check BlacklistedDids storage
		assert_eq!(BlacklistedDids::<Test>::contains_key(BLACKLISTED_DID_ONE), true);

		let blacklisted_reason_one = BlacklistedDids::<Test>::get(BLACKLISTED_DID_ONE);

		assert_eq!(blacklisted_reason_one, BLACKLISTING_REASON_ONE);

		// check BlacklistingReasons storage
		assert_eq!(BlacklistingReasons::<Test>::contains_key(REASON_CODE_ONE), true);

		let blacklisted_reason_one = BlacklistingReasons::<Test>::get(REASON_CODE_ONE);

		assert_eq!(blacklisted_reason_one, BLACKLISTING_REASON_ONE);

		// check BlacklistingReasonsRLookup storage
		assert_eq!(BlacklistingReasonsRLookup::<Test>::contains_key(BLACKLISTING_REASON_ONE), true);

		let reason_code_one = BlacklistingReasonsRLookup::<Test>::get(BLACKLISTING_REASON_ONE);

		assert_eq!(reason_code_one, REASON_CODE_ONE);

		// check ReasonsCounter storage
		assert_eq!(ReasonsCounter::<Test>::get(), 1);
  })
}

#[test]
fn test_add_extrinsic() {
	new_test_ext().execute_with(|| {
		assert_ok!(CheckAccess::add_allowed_extrinsic(
			Origin::root(),
			SECOND_PALLET_NAME,
      SECOND_FUNCTION_NAME
		));
    assert_eq!(WhitelistedPallets::<Test>::contains_key(SECOND_PALLET_NAME, SECOND_FUNCTION_NAME), true);
	})
}

#[test]
fn test_add_already_added_extrinsic() {
	new_test_ext().execute_with(|| {
		assert_eq!(WhitelistedPallets::<Test>::contains_key(FIRST_PALLET_NAME, FIRST_FUNCTION_NAME), true);
		assert_noop!(CheckAccess::add_allowed_extrinsic(
			Origin::root(),
			FIRST_PALLET_NAME,
      FIRST_FUNCTION_NAME
		), Error::<Test>::ExtrinsicAlreadyExists);
	})
}

#[test]
fn test_remove_extrinsic() {
	new_test_ext().execute_with(|| {
    assert_eq!(WhitelistedPallets::<Test>::contains_key(FIRST_PALLET_NAME, FIRST_FUNCTION_NAME), true);

    assert_ok!(CheckAccess::remove_allowed_extrinsic(
			Origin::root(),
			FIRST_PALLET_NAME,
      FIRST_FUNCTION_NAME
		));

    assert_eq!(WhitelistedPallets::<Test>::contains_key(FIRST_PALLET_NAME, FIRST_FUNCTION_NAME), false);
	})
}

#[test]
fn test_remove_non_existing_extrinsic() {
	new_test_ext().execute_with(|| {
    assert_eq!(WhitelistedPallets::<Test>::contains_key(SECOND_PALLET_NAME, SECOND_FUNCTION_NAME), false);

    assert_noop!(CheckAccess::remove_allowed_extrinsic(
			Origin::root(),
			SECOND_PALLET_NAME,
      SECOND_FUNCTION_NAME
		), Error::<Test>::ExtrinsicDoesNotExist);
	})
}

#[test]
fn test_add_blacklisted_did() {
	new_test_ext().execute_with(|| {
		assert_ok!(CheckAccess::add_blacklisted_did(
			Origin::root(),
			BLACKLISTED_DID_TWO,
			None
		));

    assert_eq!(BlacklistedDids::<Test>::contains_key(BLACKLISTED_DID_TWO), true);
		let blacklisting_reason = BlacklistedDids::<Test>::get(BLACKLISTED_DID_TWO);
		assert_eq!(blacklisting_reason, *b"Other\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
	})
}

#[test]
fn test_add_blacklisted_did_with_valid_reason_code() {
	new_test_ext().execute_with(|| {
		assert_ok!(CheckAccess::add_blacklisted_did(
			Origin::root(),
			BLACKLISTED_DID_TWO,
			Some(REASON_CODE_ONE)
		));

    assert_eq!(BlacklistedDids::<Test>::contains_key(BLACKLISTED_DID_TWO), true);
		let blacklisting_reason = BlacklistedDids::<Test>::get(BLACKLISTED_DID_TWO);
		assert_eq!(blacklisting_reason, BLACKLISTING_REASON_ONE);
	})
}

#[test]
fn test_add_blacklisted_did_with_invalid_reason_code() {
	new_test_ext().execute_with(|| {
		assert_noop!(CheckAccess::add_blacklisted_did(
			Origin::root(),
			BLACKLISTED_DID_TWO,
			Some(REASON_CODE_TWO)
		), Error::<Test>::InvalidReasonCode);
	})
}

#[test]
fn test_add_already_existing_blacklisted_did() {
	new_test_ext().execute_with(|| {
		assert_noop!(CheckAccess::add_blacklisted_did(
			Origin::root(),
			BLACKLISTED_DID_ONE,
			None
		), Error::<Test>::DidAlreadyBlacklisted);
	})
}

#[test]
fn test_remove_blacklisted_did() {
	new_test_ext().execute_with(|| {
		assert_ok!(CheckAccess::remove_blacklisted_did(
			Origin::root(),
			BLACKLISTED_DID_ONE,
		));

    assert_eq!(BlacklistedDids::<Test>::contains_key(BLACKLISTED_DID_ONE), false);
	})
}

#[test]
fn test_remove_non_existing_blacklisted_did() {
	new_test_ext().execute_with(|| {
		assert_noop!(CheckAccess::remove_blacklisted_did(
			Origin::root(),
			BLACKLISTED_DID_TWO,
		), Error::<Test>::DidIsNotBlacklisted);
	})
}

#[test]
fn test_add_blacklisting_reason() {
	new_test_ext().execute_with(|| {
		let current_reason_code = ReasonsCounter::<Test>::get();
		assert_ok!(CheckAccess::add_blacklisting_reason(
			Origin::root(),
			BLACKLISTING_REASON_TWO,
		));
    assert_eq!(ReasonsCounter::<Test>::get(), current_reason_code+1);
    assert_eq!(BlacklistingReasons::<Test>::contains_key(REASON_CODE_TWO), true);
    assert_eq!(BlacklistingReasonsRLookup::<Test>::contains_key(BLACKLISTING_REASON_TWO), true);

		let blacklisted_reason_two = BlacklistingReasons::<Test>::get(REASON_CODE_TWO);
		let reason_code_two = BlacklistingReasonsRLookup::<Test>::get(BLACKLISTING_REASON_TWO);

		assert_eq!(blacklisted_reason_two, BLACKLISTING_REASON_TWO);
		assert_eq!(reason_code_two, REASON_CODE_TWO);
	})
}

#[test]
fn test_add_already_existing_blacklisting_reason() {
	new_test_ext().execute_with(|| {
		assert_noop!(CheckAccess::add_blacklisting_reason(
			Origin::root(),
			BLACKLISTING_REASON_ONE,
		), Error::<Test>::ReasonAlreadyAdded);
	})
}

#[test]
fn test_remove_blacklisting_reason() {
	new_test_ext().execute_with(|| {
		assert_ok!(CheckAccess::remove_blacklisting_reason(
			Origin::root(),
			REASON_CODE_ONE,
		));

    assert_eq!(BlacklistingReasons::<Test>::contains_key(REASON_CODE_ONE), false);
    assert_eq!(BlacklistingReasonsRLookup::<Test>::contains_key(BLACKLISTING_REASON_ONE), false);
	})
}

#[test]
fn test_remove_non_existing_blacklisting_reason() {
	new_test_ext().execute_with(|| {
		assert_noop!(CheckAccess::remove_blacklisting_reason(
			Origin::root(),
			REASON_CODE_TWO,
		), Error::<Test>::ReasonIsNotAdded);
	})
}