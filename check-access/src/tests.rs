use crate::mock::*;
use super::*;
use frame_support::{ assert_ok };

#[test]
fn test_genesis_worked() {
	new_test_ext().execute_with(|| {
		assert_eq!(WhitelistedPallets::<Test>::contains_key(FIRST_PALLET_NAME, FIRST_FUNCTION_NAME), true);
		let empty_tuple = WhitelistedPallets::<Test>::get(FIRST_PALLET_NAME, FIRST_FUNCTION_NAME);
		assert_eq!(empty_tuple == (), true);
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
#[should_panic]
fn test_add_already_added_extrinsic() {
	new_test_ext().execute_with(|| {
		assert_eq!(WhitelistedPallets::<Test>::contains_key(FIRST_PALLET_NAME, FIRST_FUNCTION_NAME), true);
		assert_ok!(CheckAccess::add_allowed_extrinsic(
			Origin::root(),
			FIRST_PALLET_NAME,
      FIRST_FUNCTION_NAME
		));
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
#[should_panic]
fn test_remove_non_existing_extrinsic() {
	new_test_ext().execute_with(|| {
    assert_eq!(WhitelistedPallets::<Test>::contains_key(SECOND_PALLET_NAME, SECOND_FUNCTION_NAME), false);

    assert_ok!(CheckAccess::remove_allowed_extrinsic(
			Origin::root(),
			SECOND_PALLET_NAME,
      SECOND_FUNCTION_NAME
		));
	})
}