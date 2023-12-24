use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::BoundedVec;

#[test]
fn create_claim() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let ok = PoexModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		assert_ok!(ok);
	});
}

#[test]
fn create_claim_failed_when_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoexModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		let error = PoexModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		assert_noop!(error, Error::<Test>::AlreadyExist);
	});
}

#[test]
fn revoke_claim() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoexModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		let ok = PoexModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone());
		assert_ok!(ok);
	});
}

#[test]
fn revoke_claim_failed_when_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1, 2]).unwrap();
		let error = PoexModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone());
		assert_noop!(error, Error::<Test>::NotExist);
	});
}

#[test]
fn revoke_claim_failed_when_error_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoexModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
		let error = PoexModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone());
		assert_noop!(error, Error::<Test>::ErrorOwner);
	});
}

#[test]
fn transfer_claim() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let origin = RuntimeOrigin::signed(1);
		let _ = PoexModule::create_claim(origin, claim.clone());
		let origin = RuntimeOrigin::signed(1);
		let dest = 2u64;
		let ok = PoexModule::transfer_claim(origin, dest, claim.clone());
		assert_ok!(ok);
	});
}

#[test]
fn transfer_claim_failed_when_same_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let origin = RuntimeOrigin::signed(1);
		let _ = PoexModule::create_claim(origin, claim.clone());
		let origin = RuntimeOrigin::signed(1);
		let dest = 1u64;
		let error = PoexModule::transfer_claim(origin, dest, claim.clone());
		assert_noop!(error, Error::<Test>::SameOwner);
	});
}

#[test]
fn transfer_claim_failed_when_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let origin = RuntimeOrigin::signed(1);
		let dest = 2u64;
		let error = PoexModule::transfer_claim(origin, dest, claim.clone());
		assert_noop!(error, Error::<Test>::NotExist);
	});
}

#[test]
fn transfer_claim_failed_when_error_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let origin = RuntimeOrigin::signed(2);
		let _ = PoexModule::create_claim(origin, claim.clone());
		let origin = RuntimeOrigin::signed(1);
		let dest = 2u64;
		let error = PoexModule::transfer_claim(origin, dest, claim.clone());
		assert_noop!(error, Error::<Test>::ErrorOwner);
	});
}
