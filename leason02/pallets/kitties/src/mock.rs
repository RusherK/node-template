use crate::*;
use sp_core::H256;
use balances;
use frame_support::{impl_outer_origin, impl_outer_event, parameter_types, weights::Weight,
	traits::{OnFinalize, OnInitialize}
};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
};
use frame_system as system;
use sp_io;

impl_outer_origin! {
	pub enum Origin for Test {}
}

// Configure a mock runtime to test the pallet.

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	pub const ExistentialDeposit: u64 = 1;
}

impl system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	// type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	// type AccountData = ();
	type AccountData = balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();

}

impl balances::Trait for Test {
	type Balance = u64;
	type MaxLocks = ();
	type Event = TestEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = system::Module<Test>;
	type WeightInfo = ();
}

mod kitties_event {
	pub use crate::Event;
}

// balances 报错
impl_outer_event! {
	pub enum TestEvent for Test {
		system<T>,
		balances<T>,
		kitties_event<T>,
	}
}

type Randomness = pallet_randomness_collective_flip::Module<Test>;

parameter_types! {
	pub const StakingMoney: u64 = 10;
}

impl Trait for Test {
	type Event = TestEvent;
	type Randomness = Randomness;
	type KittyIndex = u32;
	type Currency = balances::Module<Self>;
	type StakingMoney = StakingMoney;
}

pub type Kitties = Module<Test>;

pub type System = system::Module<Test>;
pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		Kitties::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Kitties::on_initialize(System::block_number());
	}
}


// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	// system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	let mut t = system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();
	balances::GenesisConfig::<Test> {
		// Provide some initial balances
		balances: vec![(1, 10000000000), (2, 110000000), (3, 1200000000), (4, 1300000000), (5, 1400000000)],
	}
		.assimilate_storage(&mut t)
		.unwrap();
	let mut ext: sp_io::TestExternalities = t.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
