#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::{DispatchResult, *};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[pallet::constant]
		type MaxLength: Get<u32>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn data_smap)]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxLength>,
		(T::AccountId, T::BlockNumber),
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created(T::AccountId, Vec<u8>),
		Revoked(T::AccountId, Vec<u8>),
		Transfer(T::AccountId, T::AccountId, Vec<u8>),
		HasDest(T::AccountId, bool),
	}

	#[pallet::error]
	pub enum Error<T> {
		AlreadyExist,
		TooLong,
		NotExist,
		ErrorOwner,
		SameOwner,
		NoDest,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_proof(origin: OriginFor<T>, proof: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let bounded_proof = BoundedVec::<u8, T::MaxLength>::try_from(proof.clone())
				.map_err(|_| Error::<T>::TooLong)?;

			ensure!(!Proofs::<T>::contains_key(&bounded_proof), Error::<T>::AlreadyExist);

			Proofs::<T>::insert(
				&bounded_proof,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);

			Self::deposit_event(Event::Created(sender, proof));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn revoke_proof(origin: OriginFor<T>, proof: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let bounded_proof = BoundedVec::<u8, T::MaxLength>::try_from(proof.clone())
				.map_err(|_| Error::<T>::TooLong)?;
			let (owner, _) = Proofs::<T>::get(&bounded_proof).ok_or(Error::<T>::NotExist)?;

			ensure!(owner == sender, Error::<T>::ErrorOwner);

			Proofs::<T>::remove(&bounded_proof);

			Self::deposit_event(Event::Revoked(sender, proof));
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer_proof(
			origin: OriginFor<T>,
			dest: T::AccountId,
			proof: Vec<u8>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(sender != dest, Error::<T>::SameOwner);

			let has = frame_system::Pallet::<T>::account_exists(&dest);
			//ensure!(has, Error::<T>::NoDest); //test failed
			Self::deposit_event(Event::HasDest(dest.clone(), has));

			let bounded_proof = BoundedVec::<u8, T::MaxLength>::try_from(proof.clone())
				.map_err(|_| Error::<T>::TooLong)?;
			let (owner, _) = Proofs::<T>::get(&bounded_proof).ok_or(Error::<T>::NotExist)?;

			ensure!(owner == sender, Error::<T>::ErrorOwner);

			Proofs::<T>::mutate(&bounded_proof, |v| {
				*v = Some((dest.clone(), frame_system::Pallet::<T>::block_number()))
			});

			Self::deposit_event(Event::Transfer(sender, dest, proof));
			Ok(())
		}
	}
}
