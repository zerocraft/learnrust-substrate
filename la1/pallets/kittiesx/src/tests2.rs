use crate::{mock::*, Error, Event, Kitty};
use frame_support::{assert_noop, assert_ok, traits::fungible::Mutate};
use frame_system::Origin;

const TEST_AMOUNT: u128 = 10000;

#[test]
fn create_kitty_with_event() {
	new_test_ext().execute_with(|| {
		let account_id = 1u64;
		Balances::set_balance(&account_id, TEST_AMOUNT);
		let kitty_id = KittiesxModule::next_kitty_id();
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(account_id)));
		let kitty = KittiesxModule::kitties(kitty_id).unwrap();
		let event = Event::KittyCreated { owner: account_id, kitty_id, kitty };
		println!("{:?}", event);
		System::assert_last_event(event.into());
	});
}

#[test]
fn breed_kitty_with_event() {
	new_test_ext().execute_with(|| {
		let account_id = 1u64;
		Balances::set_balance(&account_id, TEST_AMOUNT);
		let kitty_id_1 = KittiesxModule::next_kitty_id();
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(account_id)));
		let kitty_id_2 = KittiesxModule::next_kitty_id();
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(account_id)));
		let kitty_id = KittiesxModule::next_kitty_id();
		assert_ok!(KittiesxModule::breed(
			RuntimeOrigin::signed(account_id),
			kitty_id_1,
			kitty_id_2
		));
		let kitty = KittiesxModule::kitties(kitty_id).unwrap();
		let event = Event::KittyBred { owner: account_id, kitty_id, kitty };
		println!("{:?}", event);
		System::assert_last_event(event.into());
	});
}

#[test]
fn transfer_kitty_with_event() {
	new_test_ext().execute_with(|| {
		let account_id = 1u64;
		Balances::set_balance(&account_id, TEST_AMOUNT);
		let kitty_id = KittiesxModule::next_kitty_id();
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(account_id)));
		let recipient_id = 2u64;
		assert_ok!(KittiesxModule::transfer(
			RuntimeOrigin::signed(account_id),
			recipient_id,
			kitty_id
		));
		let event = Event::KittyTransfer { owner: account_id, recipient: recipient_id, kitty_id };
		println!("{:?}", event);
		System::assert_last_event(event.into());
	});
}
