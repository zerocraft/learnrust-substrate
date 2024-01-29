//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as PoexModule;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

const SEED: u32 = 0;

#[benchmarks]
mod benchmarks {
	use super::*;
	use frame_support::{traits::Get, BoundedVec};

	#[benchmark]
	fn create_claim() {
		let max = T::MaxClaimLength::get();
		let claim = BoundedVec::try_from(vec![0; max as usize]).unwrap();
		let caller: T::AccountId = whitelisted_caller();

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), claim.clone());

		assert_eq!(Proofs::<T>::get(&claim).is_some(), true);
	}

	#[benchmark]
	fn revoke_claim() {
		let max = T::MaxClaimLength::get();
		let claim = BoundedVec::try_from(vec![0; max as usize]).unwrap();
		let caller: T::AccountId = whitelisted_caller();

		let c_claim = claim.clone();
		let _ = PoexModule::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), c_claim);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), claim.clone());

		assert_eq!(Proofs::<T>::get(&claim).is_none(), true);
	}

	#[benchmark]
	fn transfer_claim() {
		let max = T::MaxClaimLength::get();
		let claim = BoundedVec::try_from(vec![0; max as usize]).unwrap();
		let caller: T::AccountId = whitelisted_caller();

		let c_claim = claim.clone();
		let _ = PoexModule::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), c_claim);

		let dest: T::AccountId = account("dest", 0, SEED);
		#[extrinsic_call]
		_(RawOrigin::Signed(caller), dest, claim.clone());

		assert_eq!(Proofs::<T>::get(&claim).is_some(), true);
	}

	impl_benchmark_test_suite!(PoexModule, crate::mock::new_test_ext(), crate::mock::Test);
}
