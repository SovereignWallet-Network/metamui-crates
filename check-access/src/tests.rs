use crate::mock::*;
use super::*;
use frame_support::{ assert_ok };

#[test]
fn test_add_extrinsic() {
	new_test_ext().execute_with(|| {
		assert_ok!(CheckAccess::add_allowed_extrinsic(
			Origin::root(),
			PALLET_NAME,
      FUNCTION_NAME
		));

    let extrinsic = ExtrinsicsStruct { pallet_name: PALLET_NAME, function_name: FUNCTION_NAME };
    assert_eq!(WhitelistedPallets::<Test>::contains_key(extrinsic.clone()), true);
	})
}

#[test]
#[should_panic]
fn test_add_already_added_extrinsic() {
	new_test_ext().execute_with(|| {
		assert_ok!(CheckAccess::add_allowed_extrinsic(
			Origin::root(),
			PALLET_NAME,
      FUNCTION_NAME
		));

    let extrinsic = ExtrinsicsStruct { pallet_name: PALLET_NAME, function_name: FUNCTION_NAME };
    assert_eq!(WhitelistedPallets::<Test>::contains_key(extrinsic.clone()), true);

    assert_ok!(CheckAccess::add_allowed_extrinsic(
			Origin::root(),
			PALLET_NAME,
      FUNCTION_NAME
		));
	})
}

#[test]
fn test_remove_extrinsic() {
	new_test_ext().execute_with(|| {
		assert_ok!(CheckAccess::add_allowed_extrinsic(
			Origin::root(),
			PALLET_NAME,
      FUNCTION_NAME
		));

    let extrinsic = ExtrinsicsStruct { pallet_name: PALLET_NAME, function_name: FUNCTION_NAME };
    assert_eq!(WhitelistedPallets::<Test>::contains_key(extrinsic.clone()), true);

    assert_ok!(CheckAccess::remove_allowed_extrinsic(
			Origin::root(),
			PALLET_NAME,
      FUNCTION_NAME
		));

    assert_eq!(WhitelistedPallets::<Test>::contains_key(extrinsic.clone()), false);
	})
}

#[test]
#[should_panic]
fn test_remove_non_existing_extrinsic() {
	new_test_ext().execute_with(|| {
    let extrinsic = ExtrinsicsStruct { pallet_name: PALLET_NAME, function_name: FUNCTION_NAME };
    assert_eq!(WhitelistedPallets::<Test>::contains_key(extrinsic.clone()), false);

    assert_ok!(CheckAccess::remove_allowed_extrinsic(
			Origin::root(),
			PALLET_NAME,
      FUNCTION_NAME
		));
	})
}