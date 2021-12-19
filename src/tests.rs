use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

use crate::card::*;

// #[test]
// fn it_works_for_default_value() {
// 	new_test_ext().execute_with(|| {
// 		// Dispatch a signed extrinsic.
// 		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
// 		// Read pallet storage and assert an expected result.
// 		assert_eq!(TemplateModule::something(), Some(42));
// 	});
// }

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(
// 			TemplateModule::cause_error(Origin::signed(1)),
// 			Error::<Test>::NoneValue
// 		);
// 	});
// }


#[test]
fn creator_creates_card_ok() {
	new_test_ext().execute_with(|| {
		let card = Card { 
			name: vec!(1,4,45,3,2),
			card_type: CardType::Creature,
			color: RED,
			rules: vec!(1,4,45,3,2),
			image: H256([56u8; 32]),
		 };
		// signed account create
		assert_ok!(Cards::create_card_pack(Origin::signed(1), card.clone(), 10));
		// read from storage
		let id = Cards::previous_card_id();
		assert_eq!(Cards::cards(id), Some(card));
	});
}

#[test]
fn only_creator_can_create_card_error() {
	new_test_ext().execute_with(|| {
		todo!()
	})
}

#[test]
fn cannot_create_card_id_overflow_error() {
	new_test_ext().execute_with(|| {
		todo!()
	})
}

#[test]
fn creator_assigns_new_creator_ok() {
	new_test_ext().execute_with(|| {
		//assert_ok!(TemplateModule::set_creator(Origin::signed(1), ));
		todo!()
	})
}

#[test]
fn only_creator_can_assign_crearor_error() {
	new_test_ext().execute_with(|| {
		todo!()
	})
}


#[test]
fn creator_withdraws_creator_ok() {
	new_test_ext().execute_with(|| {
		todo!()
	})
}

#[test]
fn only_creator_can_withdraw_crearor_error() {
	new_test_ext().execute_with(|| {
		todo!()
	})
}

#[test]
fn cannot_withdraw_crearor_non_creator_error() {
	new_test_ext().execute_with(|| {
		todo!()
	})
}

#[test]
fn cannot_assign_crearor_already_creator_error() {
	new_test_ext().execute_with(|| {
		todo!()
	})
}