use crate as pallet_kittiesx;
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU64},
	PalletId,
};
use sp_core::{ConstU128, ConstU32, H256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;

pub const EXISTENTIAL_DEPOSIT: u128 = 1;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		KittiesxModule: pallet_kittiesx,
		Randomness:pallet_insecure_randomness_collective_flip,
		Balances: pallet_balances,
	}
);

parameter_types! {
	pub KittyPalletId:PalletId =PalletId(*b"kittiesX");
	pub KittyPledgePrice: Balance = EXISTENTIAL_DEPOSIT *10;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type MaxHolds = ();
}

impl pallet_kittiesx::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Randomness = Randomness;
	type Currency = Balances;
	type KittyCreatePrice = KittyPledgePrice;
	type PalletId = KittyPalletId;
}

impl pallet_insecure_randomness_collective_flip::Config for Test {}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities =
		frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
