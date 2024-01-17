#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_imports)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

pub mod crypto {
	use codec::alloc::string::String;
	use sp_core::{crypto::KeyTypeId, sr25519::Signature as Sr25519Signature};
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};

	pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"ocwx");

	app_crypto!(sr25519, KEY_TYPE);

	pub struct AuthId;

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for AuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for AuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use codec::{Decode, Encode};
	use frame_support::{dispatch::Vec, pallet_prelude::*};
	use frame_system::{
		offchain::{
			AppCrypto, CreateSignedTransaction, SendUnsignedTransaction, SignedPayload, Signer,
			SigningTypes,
		},
		pallet_prelude::*,
	};
	use lite_json::json::JsonValue;
	use scale_info;
	use sp_io::offchain_index;
	use sp_runtime::offchain::{self, http, storage::StorageValueRef, Duration};
	use sp_std::{prelude::*, str};

	const ONCHAIN_TX_KEY: &[u8] = b"ocwx-key-";
	const INDEXING_DATA: &[u8] = b"ocwx.indexing.data";

	#[derive(Debug, serde::Deserialize, Encode, Decode, Default)]
	struct IndexingData(u128, Vec<u8>);

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	pub struct Payload<Public, BlockNumber> {
		number: u32,
		public: Public,
		block_number: BlockNumber,
	}

	impl<T: SigningTypes> SignedPayload<T> for Payload<T::Public, BlockNumberFor<T>> {
		fn public(&self) -> T::Public {
			self.public.clone()
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	}

	#[pallet::validate_unsigned]
	#[allow(unused_variables)]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;
		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			if let Call::submit_price_unsigned_with_signed_payload {
				ref timestamp,
				ref payload,
				ref signature,
			} = call
			{
				let signature_valid =
					SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone());
				if !signature_valid {
					return InvalidTransaction::BadProof.into()
				}
				Self::validate_transaction_parameters(&payload.block_number, &payload.number)
			} else if let Call::submit_price_unsigned { block_number, price: new_price } = call {
				Self::validate_transaction_parameters(block_number, new_price)
			} else {
				InvalidTransaction::Call.into()
			}
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn numbers)]
	pub type Numbers<T> = StorageValue<_, u128>;

	#[pallet::storage]
	#[pallet::getter(fn block_keys)]
	pub type BlockKeys<T: Config> =
		StorageMap<_, Blake2_128Concat, BlockNumberFor<T>, BoundedVec<u8, ConstU32<32>>>;

	#[pallet::storage]
	#[pallet::getter(fn payload_numbers)]
	pub type PayloadNumbers<T: Config> = StorageMap<_, Blake2_128Concat, u64, (u32, bool)>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NumberStored { number: u128, who: T::AccountId, block_number: BlockNumberFor<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn storage_number(origin: OriginFor<T>, number: u128) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let block_number = frame_system::Pallet::<T>::block_number();
			let key = Self::derive_block_number_to_key(block_number);

			Numbers::<T>::put(number);
			Self::deposit_event(Event::NumberStored { number, who, block_number });

			let data = IndexingData(number, INDEXING_DATA.to_vec());
			offchain_index::set(&key, &data.encode());

			let bk: BoundedVec<u8, ConstU32<32>> = BoundedVec::truncate_from(key);
			BlockKeys::<T>::insert(block_number, bk);

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn submit_price_unsigned_with_signed_payload(
			origin: OriginFor<T>,
			timestamp: u64,
			payload: Payload<T::Public, BlockNumberFor<T>>,
			_signature: T::Signature,
		) -> DispatchResult {
			ensure_none(origin)?;

			PayloadNumbers::<T>::insert(timestamp, (payload.number, true));
			log::info!(
				"=== call submit_price_unsigned_with_signed_payload === {:?}",
				payload.number
			);

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn submit_price_unsigned(
			origin: OriginFor<T>,
			_block_number: BlockNumberFor<T>,
			_price: u32,
		) -> DispatchResultWithPostInfo {
			ensure_none(origin)?;
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn derive_block_number_to_key(n: BlockNumberFor<T>) -> Vec<u8> {
			n.using_encoded(|encode| {
				ONCHAIN_TX_KEY.iter().chain(encode).copied().collect::<Vec<u8>>()
			})
		}

		#[allow(dead_code)]
		fn log_indexing_data(n: BlockNumberFor<T>) {
			log::info!("=== log_indexing_data === {:?}", n);
			let key = Self::derive_block_number_to_key(n - 1u32.into());
			let oci_mem = StorageValueRef::persistent(&key);
			if let Ok(Some(data)) = oci_mem.get::<IndexingData>() {
				log::info!("=== indexing data === {:?} : {:?}", &key, data);
				if let Ok(utf8str) = str::from_utf8(&data.1) {
					log::info!("=== utf8str === {:?}", utf8str);
				}
			}
		}

		#[allow(dead_code)]
		fn call_fetch_price(n: BlockNumberFor<T>) {
			log::info!("=== call_fetch_price === {:?}", n);
			if let Ok(number) = Self::fetch_price() {
				let timestamp = sp_io::offchain::timestamp().unix_millis();
				let signer = Signer::<T, T::AuthorityId>::any_account();
				log::info!("=== call_fetch_price timestamp === {:?}", timestamp);
				if let Some((_, res)) = signer.send_unsigned_transaction(
					|acct| Payload { number, public: acct.public.clone(), block_number: n },
					|payload, signature| Call::submit_price_unsigned_with_signed_payload {
						timestamp,
						payload,
						signature,
					},
				) {
					match res {
						Ok(()) => {
							log::info!("=== call_fetch_price successfully sent ===");
						},
						Err(()) => {
							log::error!("=== call_fetch_price sending failed ===");
						},
					};
				} else {
					log::error!("=== No local account available ===");
				}
			}
		}

		#[allow(dead_code)]
		fn fetch_price() -> Result<u32, http::Error> {
			let url = "https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD";
			log::info!("=== fetch_price === {:?}", url);

			let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(6_000));

			let request = http::Request::get(url);
			let pending = request.deadline(deadline).send().map_err(|_| http::Error::IoError)?;

			let response =
				pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
			if response.code != 200 {
				log::warn!("Unexpected status code: {}", response.code);
				return Err(http::Error::Unknown)
			}
			let body = response.body().collect::<Vec<u8>>();

			let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
				log::warn!("No UTF8 body");
				http::Error::Unknown
			})?;

			let price = match Self::parse_price(body_str) {
				Some(price) => Ok(price),
				None => {
					log::warn!("Unable to extract price from the response: {:?}", body_str);
					Err(http::Error::Unknown)
				},
			}?;

			log::warn!("Got price: {} cents", price);

			Ok(price)
		}

		#[allow(dead_code)]
		fn parse_price(price_str: &str) -> Option<u32> {
			let val = lite_json::parse_json(price_str);
			let price = match val.ok()? {
				JsonValue::Object(obj) => {
					let (_, v) =
						obj.into_iter().find(|(k, _)| k.iter().copied().eq("USD".chars()))?;
					match v {
						JsonValue::Number(number) => number,
						_ => return None,
					}
				},
				_ => return None,
			};
			let exp = price.fraction_length.saturating_sub(2);
			Some(price.integer as u32 * 100 + (price.fraction / 10_u64.pow(exp)) as u32)
		}

		#[allow(dead_code)]
		fn validate_transaction_parameters(
			_block_number: &BlockNumberFor<T>,
			_new_price: &u32,
		) -> TransactionValidity {
			ValidTransaction::with_tag_prefix("ExampleOffchainWorker")
				.longevity(5)
				.propagate(true)
				.build()
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(n: BlockNumberFor<T>) {
			log::info!("=== offchain_worker === {:?}", n);
			//Self::log_indexing_data(n);
			Self::call_fetch_price(n);
		}
	}
}
