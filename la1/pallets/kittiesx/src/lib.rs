#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_imports)]

pub use pallet::*;

mod migrations;

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
		traits::{Currency, ExistenceRequirement, Randomness, ReservableCurrency, StorageVersion},
		Blake2_128Concat, PalletId,
	};
	use frame_system::{
		pallet,
		pallet_prelude::{BlockNumberFor, *},
	};
	use sp_io::hashing::blake2_128;
	use sp_runtime::traits::AccountIdConversion;

	pub type KittyId = u32;

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(
		Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
	)]
	pub struct Kitty {
		pub dna: [u8; 16],
		pub name: [u8; 8],
		pub feature: [u8; 5],
	}

	pub const DEFAULT_KITTY_FEATURE: [u8; 5] = *b"happy";
	pub const DEFAULT_KITTY_NAME: [u8; 8] = *b"mimimimi";
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;

		type Currency: ReservableCurrency<Self::AccountId>;
		#[pallet::constant]
		type KittyCreatePrice: Get<BalanceOf<Self>>;
		type PalletId: Get<PalletId>;
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

	#[pallet::storage]
	#[pallet::getter(fn kitty_on_sale)]
	pub type KittyOnSale<T: Config> =
		StorageMap<_, Blake2_128Concat, KittyId, BalanceOf<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated { owner: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyBred { owner: T::AccountId, kitty_id: KittyId, kitty: Kitty },
		KittyTransfer { owner: T::AccountId, recipient: T::AccountId, kitty_id: KittyId },
		KittyOnSale { owner: T::AccountId, kitty_id: KittyId, price: BalanceOf<T> },
		KittyBought { owner: T::AccountId, kitty_id: KittyId },
	}

	#[pallet::error]
	pub enum Error<T> {
		KittyOverflow,
		InvalidKittyId,
		KittySingleParent,
		ErrorKittyOwner,
		ErrprRecipient,
		KittyOnSale,
		NotEnoughCurrency,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_runtime_upgrade() -> Weight {
			let mut _w = migrations::v1::upgrade::<T>();
			_w = migrations::v2::upgrade::<T>();
			_w
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_id = Self::get_next_id()?;
			let kitty = Kitty {
				dna: Self::random_value(&who),
				name: DEFAULT_KITTY_NAME,
				feature: DEFAULT_KITTY_FEATURE,
			};

			let create_price = T::KittyCreatePrice::get();
			ensure!(T::Currency::can_reserve(&who, create_price), Error::<T>::NotEnoughCurrency);
			T::Currency::transfer(
				&who,
				&Self::get_pallet_account_id(),
				create_price,
				ExistenceRequirement::KeepAlive,
			)?;

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
				data[i] = (kitty_p_1.dna[i] & selector[i]) | (kitty_p_2.dna[i] & !selector[i]);
			}
			let kitty =
				Kitty { dna: data, name: DEFAULT_KITTY_NAME, feature: DEFAULT_KITTY_FEATURE };

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
			ensure!(!KittyOnSale::<T>::contains_key(kitty_id), Error::<T>::KittyOnSale);

			let owner = Self::kitty_owners(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
			ensure!(who == owner, Error::<T>::ErrorKittyOwner);

			KittyOwners::<T>::insert(kitty_id, &recipient);

			Self::deposit_event(Event::KittyTransfer { owner, recipient, kitty_id });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn sale(
			origin: OriginFor<T>,
			kitty_id: KittyId,
			price: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Kitties::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);
			let owner = Self::kitty_owners(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
			ensure!(who == owner, Error::<T>::ErrorKittyOwner);
			ensure!(!KittyOnSale::<T>::contains_key(kitty_id), Error::<T>::KittyOnSale);

			KittyOnSale::<T>::insert(kitty_id, price);

			Self::deposit_event(Event::KittyOnSale { owner, kitty_id, price });
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn buy(origin: OriginFor<T>, kitty_id: KittyId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let kitty_on_sale = Self::kitty_on_sale(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
			let owner = Self::kitty_owners(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;

			let price = kitty_on_sale;
			ensure!(T::Currency::can_reserve(&who, price), Error::<T>::NotEnoughCurrency);

			KittyOwners::<T>::insert(kitty_id, &who);
			KittyOnSale::<T>::remove(kitty_id);

			T::Currency::transfer(&who, &owner, price, ExistenceRequirement::KeepAlive)?;

			Self::deposit_event(Event::KittyBought { owner: who, kitty_id });
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

		fn get_pallet_account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}
	}
}
