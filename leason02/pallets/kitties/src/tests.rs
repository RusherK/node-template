
use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};




#[test]
fn owned_kitties_can_append_values(){
	new_test_ext().execute_with( || {
		run_to_block(10);
		assert_eq!(Kitties::create(Origin::signed(1),), Ok(()));
	})
}

