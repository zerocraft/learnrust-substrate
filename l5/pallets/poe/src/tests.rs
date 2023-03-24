use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_std::prelude::*;

#[test]
fn create() {
	new_test_ext().execute_with(|| {
		let proof = vec![3, 5, 7];
		let proof1 = vec![0; 513];
		let account = RuntimeOrigin::signed(1);

		assert_noop!(
			PoeModule::create_proof(account.clone(), proof1.clone()),
			Error::<Test>::TooLong
		);
		assert_ok!(PoeModule::create_proof(account.clone(), proof.clone()));
		assert_noop!(
			PoeModule::create_proof(account.clone(), proof.clone()),
			Error::<Test>::AlreadyExist
		);

		let bounded_proof = BoundedVec::try_from(proof.clone()).unwrap();
		let (owner, _) = PoeModule::data_smap(&bounded_proof).unwrap();
		assert_eq!(owner, 1);
	});
}

#[test]
fn revoke() {
	new_test_ext().execute_with(|| {
		let proof = vec![3, 5, 7];
		let proof1 = vec![0; 513];
		let proof2 = vec![4, 6, 8];
		let account = RuntimeOrigin::signed(1);
		let account1 = RuntimeOrigin::signed(2);

		assert_ok!(PoeModule::create_proof(account.clone(), proof.clone()));
		assert_noop!(
			PoeModule::revoke_proof(account.clone(), proof1.clone()),
			Error::<Test>::TooLong
		);
		assert_noop!(
			PoeModule::revoke_proof(account.clone(), proof2.clone()),
			Error::<Test>::NotExist
		);
		assert_noop!(
			PoeModule::revoke_proof(account1.clone(), proof.clone()),
			Error::<Test>::ErrorOwner
		);
		assert_ok!(PoeModule::revoke_proof(account.clone(), proof.clone()));
	});
}

#[test]
fn transfer() {
	new_test_ext().execute_with(|| {
		let proof = vec![3, 5, 7];
		let proof1 = vec![0; 513];
		let proof2 = vec![4, 6, 8];
		let account = RuntimeOrigin::signed(1);
		let account1 = RuntimeOrigin::signed(2);

		assert_ok!(PoeModule::create_proof(account.clone(), proof.clone()));
		assert_noop!(
			PoeModule::transfer_proof(account.clone(), 2, proof1.clone()),
			Error::<Test>::TooLong
		);
		assert_noop!(
			PoeModule::transfer_proof(account.clone(), 2, proof2.clone()),
			Error::<Test>::NotExist
		);
		assert_noop!(
			PoeModule::transfer_proof(account1.clone(), 1, proof.clone()),
			Error::<Test>::ErrorOwner
		);
		assert_ok!(PoeModule::transfer_proof(account.clone(), 2, proof.clone()));

		let bounded_proof = BoundedVec::try_from(proof.clone()).unwrap();
		let (owner, _) = PoeModule::data_smap(&bounded_proof).unwrap();
		assert_eq!(owner, 2);
	});
}
