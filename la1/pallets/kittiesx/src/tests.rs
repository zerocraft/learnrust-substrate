use crate::{mock::*, Error, Event, Kitty};
use frame_support::{assert_noop, assert_ok};
use frame_system::Origin;

#[test]
fn it_test() {
	assert_eq!(1, 1);
}

#[test]
fn create_kitty() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1u64;
		let origin = RuntimeOrigin::signed(account_id);
		assert_eq!(KittiesxModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesxModule::create(origin));
		assert_eq!(KittiesxModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesxModule::kitties(kitty_id).is_none(), false);
		assert_eq!(KittiesxModule::kitty_owners(kitty_id).is_none(), false);
		assert_eq!(KittiesxModule::kitty_parents(kitty_id).is_none(), true);
	});
}

#[test]
fn create_kitty_failed_when_id_overflow() {
	new_test_ext().execute_with(|| {
		crate::NextKittyId::<Test>::set(crate::KittyId::MAX);
		let origin = RuntimeOrigin::signed(1);
		assert_noop!(KittiesxModule::create(origin), Error::<Test>::KittyOverflow);
	});
}

#[test]
fn breed_kitty() {
	new_test_ext().execute_with(|| {
		let kitty_id_1 = KittiesxModule::next_kitty_id();
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(1)));
		let kitty_id_2 = KittiesxModule::next_kitty_id();
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(2)));
		let kitty_id = KittiesxModule::next_kitty_id();
		let origin = RuntimeOrigin::signed(1);
		assert_ok!(KittiesxModule::breed(origin, kitty_id_1, kitty_id_2));
		assert_eq!(KittiesxModule::kitties(kitty_id).is_none(), false);
		assert_eq!(KittiesxModule::kitty_owners(kitty_id).is_none(), false);
		assert_eq!(KittiesxModule::kitty_parents(kitty_id).is_none(), false);
	});
}

#[test]
fn breed_kitty_failed_when_kitty_single_parent() {
	new_test_ext().execute_with(|| {
		let kitty_id_1 = KittiesxModule::next_kitty_id();
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(1)));
		let origin = RuntimeOrigin::signed(1);
		assert_noop!(
			KittiesxModule::breed(origin, kitty_id_1, kitty_id_1),
			Error::<Test>::KittySingleParent
		);
	});
}

#[test]
fn breed_kitty_failed_when_kitty_invalid_kitty_id() {
	new_test_ext().execute_with(|| {
		let kitty_id_1 = 1;
		let kitty_id_2 = 2;
		let origin = RuntimeOrigin::signed(1);
		assert_noop!(
			KittiesxModule::breed(origin, kitty_id_1, kitty_id_2),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn transfer_kitty() {
	new_test_ext().execute_with(|| {
		let kitty_id = KittiesxModule::next_kitty_id();
		let origin_id = 1u64;
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(origin_id)));
		let recipient_id = 2u64;
		assert_ok!(KittiesxModule::transfer(
			RuntimeOrigin::signed(origin_id),
			recipient_id,
			kitty_id
		));
		assert_eq!(KittiesxModule::kitty_owners(kitty_id), Some(recipient_id));
	});
}

#[test]
fn transfer_kitty_failed_when_errpr_recipient() {
	new_test_ext().execute_with(|| {
		let kitty_id = KittiesxModule::next_kitty_id();
		let origin_id = 1u64;
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(origin_id)));
		assert_noop!(
			KittiesxModule::transfer(RuntimeOrigin::signed(origin_id), origin_id, kitty_id),
			Error::<Test>::ErrprRecipient
		);
	});
}

#[test]
fn transfer_kitty_failed_when_invalid_kitty_id() {
	new_test_ext().execute_with(|| {
		let kitty_id = 1;
		let origin_id = 1u64;
		let recipient_id = 2u64;
		assert_noop!(
			KittiesxModule::transfer(RuntimeOrigin::signed(origin_id), recipient_id, kitty_id),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn transfer_kitty_failed_when_error_kitty_owner() {
	new_test_ext().execute_with(|| {
		let kitty_id = KittiesxModule::next_kitty_id();
		let origin_id = 1u64;
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(origin_id)));
		let recipient_id = 2u64;
		assert_noop!(
			KittiesxModule::transfer(RuntimeOrigin::signed(recipient_id), origin_id, kitty_id),
			Error::<Test>::ErrorKittyOwner
		);
	});
}
