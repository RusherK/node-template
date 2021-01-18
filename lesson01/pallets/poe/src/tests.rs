use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, StorageMap};

#[test]
fn creat_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		assert_eq!(
			Proofs::<Test>::get(&claim),
			(1, frame_system::Module::<Test>::block_number())
		);
	})
}

#[test]
fn creat_claim_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimAlreadyClaimed
		);
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_ok!(PoeModule::transfer_claim(
			Origin::signed(1),
			claim.clone(),
			2
		));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			(2, None, system::Module::<Test>::block_number())
		);
	})
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		
		assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
            Error::<Test>::ClaimAlreadyClaimed
        );
	})
}

#[test]
fn transfer_claim_failed_with_wrong_owner(){
	new_test_ext().execute_with(||{
		let claim = vec![0,1 ];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
            Error::<Test>::ClaimAlreadyClaimed
        );
	})
}
