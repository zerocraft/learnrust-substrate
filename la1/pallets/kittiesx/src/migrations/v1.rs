use frame_support::{
	pallet_prelude::*, storage::StoragePrefixedMap, traits::GetStorageVersion, weights::Weight,
};

use frame_support::{migration::storage_key_iter, Blake2_128Concat};
use frame_system::pallet_prelude::*;

use crate::{Kitties, Kitty, KittyId, Pallet, DEFAULT_KITTY_FEATURE, DEFAULT_KITTY_NAME};

#[derive(
	Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
)]
pub struct KittyV0(pub [u8; 16]);

pub fn upgrade<T: crate::Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();
	let current_version = Pallet::<T>::current_storage_version();

	if on_chain_version != 0 {
		return Weight::zero();
	}

	if current_version != 2 {
		return Weight::zero();
	}

	let module = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();

	for (kitty_id, kitty) in
		storage_key_iter::<KittyId, KittyV0, Blake2_128Concat>(module, item).drain()
	{
		let new_kitty =
			Kitty { dna: kitty.0, name: DEFAULT_KITTY_NAME, feature: DEFAULT_KITTY_FEATURE };
		Kitties::<T>::insert(kitty_id, new_kitty);
	}

	Weight::zero()
}
