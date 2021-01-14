
use crate::{mock::*, Event};
use frame_support::{assert_ok};
use frame_system::{EventRecord, Phase};

#[test]
fn owned_kitties_can_append_values(){
	new_test_ext().execute_with( || {
		run_to_block(10);
		assert_eq!(Kitties::create(Origin::signed(1),), Ok(()));
	})
}

#[test]
fn is_create_works(){
	new_test_ext().execute_with( || {
		run_to_block(10);

		assert_ok!(Kitties::create(Origin::signed(1)));

		//test event
		assert_eq!(
			System::events(),
			vec![EventRecord {
				phase: Phase::Initialization,
				event: TestEvent::kitties_event( Event::<Test>::Created( 1 as u64 , 0) ),
				topics: vec![],
			}]
		);
	})
}

#[test]
fn is_transfer_works(){
	new_test_ext().execute_with( || {
		run_to_block(10);

		Kitties::create(Origin::signed(1));

		assert_ok!(Kitties::transfer(Origin::signed(1), 3, 0));
	})
}

#[test]
fn is_breed_works(){
	new_test_ext().execute_with( || {
		run_to_block(10);

		Kitties::create(Origin::signed(1));
		Kitties::create(Origin::signed(1));

		assert_ok!(Kitties::breed(Origin::signed(1), 0, 1));

	})
}
