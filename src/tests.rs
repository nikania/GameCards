use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

use crate::card::*;

#[test]
fn creator_creates_card_ok() {
	new_test_ext().execute_with(|| {
		let card = new_card();
		// signed account create
		assert_ok!(Cards::create_card_pack(Origin::signed(ALICE), card.clone(), 10));
		// read from storage
		let id = Cards::previous_card_id();
		assert_eq!(Cards::cards(id), Some(card));
	});
}

/// creates 10 cards
fn create_card_pack(creator: Origin) -> u32 {
	let card = new_card();
	// signed account create
	assert_ok!(Cards::create_card_pack(creator, card.clone(), 10));
	Cards::previous_card_id()
}

fn new_card() -> Card {
	Card { 
		name: vec!(1,4,45,3,2),
		card_type: CardType::Creature,
		color: RED&BLACK,
		rules: vec!(1,4,45,3,2),
		image: H256([56u8; 32]),
	 }
}

#[test]
fn only_creator_can_create_card_error() {
	new_test_ext().execute_with(|| {
		let id = Cards::previous_card_id();
		let card = new_card();
		assert_noop!(Cards::create_card_pack(Origin::signed(BOB), card.clone(), 10), 
			Error::<Test>::NoPermission);
		assert_eq!(id, Cards::previous_card_id());
	})
}

#[test]
#[ignore]
fn cannot_create_card_id_overflow_error() {
	new_test_ext().execute_with(|| {
		todo!()
	})
}

#[test]
fn creator_assigns_new_creator_ok() {
	new_test_ext().execute_with(|| {
		assert_ok!(Cards::set_creator(Origin::signed(ALICE), BOB));
		assert!(crate::is_creator::<Test>(BOB));
		assert_ok!(Cards::create_card_pack(Origin::signed(BOB), new_card(), 10));
	})
}

#[test]
fn only_creator_can_assign_crearor_error() {
	new_test_ext().execute_with(|| {
		assert_noop!(Cards::set_creator(Origin::signed(BOB), MIRA), Error::<Test>::NoPermission);
		assert!(crate::is_creator::<Test>(MIRA) != true);
	})
}

#[test]
fn creator_withdraws_creator_ok() {
	new_test_ext().execute_with(|| {
		assert_ok!(Cards::set_creator(Origin::signed(ALICE), BOB));
		assert!(crate::is_creator::<Test>(BOB));
		assert_ok!(Cards::withdraw_creator(Origin::signed(ALICE), BOB));
		assert!(crate::is_creator::<Test>(BOB) != true);
		assert_noop!(Cards::create_card_pack(Origin::signed(BOB), new_card(), 10), 
			Error::<Test>::NoPermission);	})
}

#[test]
fn only_creator_can_withdraw_crearor_error() {
	new_test_ext().execute_with(|| {
		assert_ok!(Cards::set_creator(Origin::signed(ALICE), BOB));
		assert!(crate::is_creator::<Test>(BOB));
		assert_noop!(Cards::withdraw_creator(Origin::signed(MIRA), BOB), 
			Error::<Test>::NoPermission);
		assert!(crate::is_creator::<Test>(BOB));
		assert_ok!(Cards::create_card_pack(Origin::signed(BOB), new_card(), 10));	
		})
}

#[test]
fn cannot_withdraw_crearor_non_creator_error() {
	new_test_ext().execute_with(|| {
		assert_noop!(Cards::withdraw_creator(Origin::signed(ALICE), BOB), 
			Error::<Test>::AccountNotCreator);
	})
}

#[test]
fn cannot_assign_crearor_already_creator_error() {
	new_test_ext().execute_with(|| {
		assert_ok!(Cards::set_creator(Origin::signed(ALICE), BOB));
		assert!(crate::is_creator::<Test>(BOB));
		assert_noop!(Cards::set_creator(Origin::signed(ALICE), BOB), 
			Error::<Test>::AccountAlreadyCreator);
	})
}

#[test]
fn transfer_card_ok() {
	new_test_ext().execute_with(|| {
		let creator = Origin::signed(ALICE);
		let user  = 2u64;
		let card_id = create_card_pack(creator.clone());
		let _ = Cards::transfer(creator, card_id, user);

		assert_eq!(Cards::owners(2, card_id), Some(1));
	})
}

#[test]
fn transfer_card_not_owned_error() {
	new_test_ext().execute_with(|| {
		let creator1 = Origin::signed(ALICE);
		let creator2 = Origin::signed(2);
		let user  = 3u64;
		let card_id = create_card_pack(creator1.clone());
		assert_noop!(Cards::transfer(creator2, card_id, user), Error::<Test>::CardNotOwned) ;

		assert_eq!(Cards::owners(1, card_id), Some(10));
		assert_eq!(Cards::owners(2, card_id), None);
		assert_eq!(Cards::owners(3, card_id), None);
	})
}