#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_imports)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests2;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet,
		pallet_prelude::{OptionQuery, StorageValue, ValueQuery, *},
		traits::Randomness,
		Blake2_128Concat,
	};
	use frame_system::pallet_prelude::{BlockNumberFor, *};
	use sp_io::hashing::blake2_128;

	/// KittyId
	pub type KittyId = u32;
	/// Kitty Struct
	#[derive(
		Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
	)]
	pub struct Kitty(pub [u8; 16]);

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
	}

	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T> = StorageValue<_, KittyId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyId, Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owners)]
	pub type KittyOwners<T: Config> = StorageMap<_, Blake2_128Concat, KittyId, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_parents)]
	pub type KittyParents<T: Config> =
		StorageMap<_, Blake2_128Concat, KittyId, (KittyId, KittyId), OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated { owner: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyBred { owner: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyTransfer { owner: T::AccountId, recipient: T::AccountId, kitty_id: KittyId },
	}

	#[pallet::error]
	pub enum Error<T> {
		KittyOverflow,
		InvalidKittyId,
		KittySingleParent,
		ErrorKittyOwner,
		ErrprRecipient,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_id = Self::get_next_id()?;
			let kitty = Kitty(Self::random_value(&who));

			Kitties::<T>::insert(kitty_id, kitty);
			KittyOwners::<T>::insert(kitty_id, &who);

			Self::deposit_event(Event::KittyCreated { owner: who, kitty_id, kitty });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: KittyId,
			kitty_id_2: KittyId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::KittySingleParent);
			ensure!(Kitties::<T>::contains_key(kitty_id_1), Error::<T>::InvalidKittyId);
			ensure!(Kitties::<T>::contains_key(kitty_id_2), Error::<T>::InvalidKittyId);

			let kitty_id = Self::get_next_id()?;

			let kitty_p_1 = Self::kitties(kitty_id_1).unwrap();
			let kitty_p_2 = Self::kitties(kitty_id_2).unwrap();
			let selector = Self::random_value(&who);
			let mut data = [0u8; 16];
			for i in 0..selector.len() {
				data[i] = (kitty_p_1.0[i] & selector[i]) | (kitty_p_2.0[i] & !selector[i]);
			}
			let kitty = Kitty(data);

			Kitties::<T>::insert(kitty_id, kitty);
			KittyOwners::<T>::insert(kitty_id, &who);
			KittyParents::<T>::insert(kitty_id, (kitty_id_1, kitty_id_2));

			Self::deposit_event(Event::KittyBred { owner: who, kitty_id, kitty });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn transfer(
			origin: OriginFor<T>,
			recipient: T::AccountId,
			kitty_id: KittyId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(who != recipient, Error::<T>::ErrprRecipient);
			ensure!(Kitties::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);

			let owner = Self::kitty_owners(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
			ensure!(who == owner, Error::<T>::ErrorKittyOwner);

			KittyOwners::<T>::insert(kitty_id, &recipient);

			Self::deposit_event(Event::KittyTransfer { owner, recipient, kitty_id });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn get_next_id() -> Result<KittyId, DispatchError> {
			NextKittyId::<T>::try_mutate(|next_id| -> Result<KittyId, DispatchError> {
				let current_id = *next_id;
				*next_id = next_id.checked_add(1).ok_or(Error::<T>::KittyOverflow)?;
				Ok(current_id)
			})
		}

		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			//<pallet_insecure_randomness_collective_flip::Pallet<T>>::random_seed(),
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}
	}
}
