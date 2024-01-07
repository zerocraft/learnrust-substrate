use crate::{mock::*, Error, Event, Kitty};
use frame_support::{assert_noop, assert_ok, traits::fungible::Mutate};
use frame_system::Origin;

const TEST_AMOUNT: u128 = 10000;

#[test]
fn balance_work() {
	new_test_ext().execute_with(|| {
		let account_id = 1u64;
		let raw_balance = Balances::free_balance(account_id);
		Balances::set_balance(&account_id, TEST_AMOUNT);
		let set_balance = Balances::free_balance(account_id);
		println!("Balance:{:?}->{:?}", raw_balance, set_balance);
	});
}

#[test]
fn create_kitty() {
	new_test_ext().execute_with(|| {
		let account_id = 1u64;
		let origin = RuntimeOrigin::signed(account_id);
		Balances::set_balance(&account_id, TEST_AMOUNT);

		let kitty_id = KittiesxModule::next_kitty_id();
		assert_eq!(KittiesxModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesxModule::create(origin));
		assert_eq!(KittiesxModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesxModule::kitties(kitty_id).is_none(), false);
		assert_eq!(KittiesxModule::kitty_owners(kitty_id).is_none(), false);
		assert_eq!(KittiesxModule::kitty_parents(kitty_id).is_none(), true);
	});
}

#[test]
fn create_kitty_failed_when_not_enough_currency() {
	new_test_ext().execute_with(|| {
		let account_id = 1u64;
		let origin = RuntimeOrigin::signed(account_id);
		assert_noop!(KittiesxModule::create(origin), Error::<Test>::NotEnoughCurrency);
	});
}

#[test]
fn create_kitty_failed_when_id_overflow() {
	new_test_ext().execute_with(|| {
		let account_id = 1u64;
		crate::NextKittyId::<Test>::set(crate::KittyId::MAX);
		let origin = RuntimeOrigin::signed(account_id);
		Balances::set_balance(&account_id, TEST_AMOUNT);

		assert_noop!(KittiesxModule::create(origin), Error::<Test>::KittyOverflow);
	});
}

#[test]
fn breed_kitty() {
	new_test_ext().execute_with(|| {
		let account_id = 1u64;
		Balances::set_balance(&account_id, TEST_AMOUNT);
		let kitty_id_1 = KittiesxModule::next_kitty_id();
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(account_id)));

		let account_id = 2u64;
		Balances::set_balance(&account_id, TEST_AMOUNT);
		let kitty_id_2 = KittiesxModule::next_kitty_id();
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(account_id)));

		let origin = RuntimeOrigin::signed(account_id);
		let new_kitty_id = KittiesxModule::next_kitty_id();
		assert_ok!(KittiesxModule::breed(origin, kitty_id_1, kitty_id_2));
		assert_eq!(KittiesxModule::kitties(new_kitty_id).is_none(), false);
		assert_eq!(KittiesxModule::kitty_owners(new_kitty_id).is_none(), false);
		assert_eq!(KittiesxModule::kitty_parents(new_kitty_id).is_none(), false);
	});
}

#[test]
fn breed_kitty_failed_when_kitty_single_parent() {
	new_test_ext().execute_with(|| {
		let account_id = 1u64;
		Balances::set_balance(&account_id, TEST_AMOUNT);
		let kitty_id_1 = KittiesxModule::next_kitty_id();
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(account_id)));

		let origin = RuntimeOrigin::signed(account_id);
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
		assert_eq!(KittiesxModule::kitty_owners(kitty_id), Some(recipient_id));
	});
}

#[test]
fn transfer_kitty_failed_when_errpr_recipient() {
	new_test_ext().execute_with(|| {
		let kitty_id = KittiesxModule::next_kitty_id();
		let account_id = 1u64;
		Balances::set_balance(&account_id, TEST_AMOUNT);
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(account_id)));
		assert_noop!(
			KittiesxModule::transfer(RuntimeOrigin::signed(account_id), account_id, kitty_id),
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
		let account_id = 1u64;
		Balances::set_balance(&account_id, TEST_AMOUNT);
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(account_id)));
		let recipient_id = 2u64;
		assert_noop!(
			KittiesxModule::transfer(RuntimeOrigin::signed(recipient_id), account_id, kitty_id),
			Error::<Test>::ErrorKittyOwner
		);
	});
}

#[test]
fn sale_kitty() {
	new_test_ext().execute_with(|| {
		let account_id = 1u64;
		Balances::set_balance(&account_id, TEST_AMOUNT);
		let kitty_id = KittiesxModule::next_kitty_id();
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(account_id)));
		let sale_price = 1234;
		assert_ok!(KittiesxModule::sale(RuntimeOrigin::signed(account_id), kitty_id, sale_price));
		let on_sale_price = KittiesxModule::kitty_on_sale(kitty_id).unwrap();
		assert_eq!(on_sale_price, sale_price);
	});
}

#[test]
fn buy_kitty() {
	new_test_ext().execute_with(|| {
		let account_1_id = 1u64;
		Balances::set_balance(&account_1_id, TEST_AMOUNT);
		let kitty_id = KittiesxModule::next_kitty_id();
		assert_ok!(KittiesxModule::create(RuntimeOrigin::signed(account_1_id)));

		let sale_price = 1234;
		assert_ok!(KittiesxModule::sale(RuntimeOrigin::signed(account_1_id), kitty_id, sale_price));

		let account_2_id = 2u64;
		Balances::set_balance(&account_2_id, TEST_AMOUNT);

		assert_ok!(KittiesxModule::buy(RuntimeOrigin::signed(account_2_id), kitty_id));

		let account_1_free_balance = Balances::free_balance(account_1_id);
		let account_2_free_balance = Balances::free_balance(account_2_id);
		assert_eq!(account_1_free_balance, TEST_AMOUNT - 10 + sale_price);
		assert_eq!(account_2_free_balance, TEST_AMOUNT - sale_price);
		println!("account 1:{:?} account 2:{:?}", account_1_free_balance, account_2_free_balance);
	});
}
