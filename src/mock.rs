//use crate as pallet_gamecards;
use super::*;

use sp_core::H256;
use frame_support::parameter_types;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};
use frame_system as system;
//use frame_system::GenesisConfig;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		//TemplateModule: pallet_template::{Module, Call, Storage, Event<T>},
	//	Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		Cards: pallet::{Module, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
	pub const ExistentialDeposit: u64 = 1;
}

impl system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();//pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
}

impl pallet::Config for Test {
	type Event = Event;
}

// impl pallet_balances::Config for Test {
// 	type MaxLocks = ();
// 	type Balance = u64;
// 	type Event = Event;
// 	type DustRemoval = ();
// 	type ExistentialDeposit = ExistentialDeposit;
// 	type AccountStore = System;
// 	type WeightInfo = ();
// 	// type MaxReserves = ();
// 	// type ReserveIdentifier = ();
// }

pub type AccountId = u64;
pub const ALICE: AccountId = 1;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = frame_system::GenesisConfig::default()
	.build_storage::<Test>().unwrap();

	let config = pallet::GenesisConfig::<Test> {
		creator: ALICE
	};

	config.assimilate_storage(&mut storage).unwrap();
	
	let mut t: sp_io::TestExternalities = storage.into();

	t.execute_with(|| System::set_block_number(1) );
	t
}


// // Build genesis storage according to the mock runtime.
// pub fn new_test_ext() -> sp_io::TestExternalities {
// 	let mut t: sp_io::TestExternalities = frame_system::GenesisConfig::default()
// 		.build_storage::<Test>().unwrap().into();
// 	t.execute_with(|| System::set_block_number(1) );
// 	t
// 	// frame_system::GenesisConfig::default()
// 	// 	.build_storage::<Test>()
// 	// 	.unwrap().into()

// 	// 	pallet_gamecards::GenesisConfig::<Test> {
// 	// 	// Provide some initial balances
// 	// 	balances: ROLES.iter().map(|x| (x.0, 100000)).collect(),
// 	// }
// 	// .assimilate_storage(&mut t)
// 	// .unwrap();

// 	// super::GenesisConfig::<TestRuntime> {
// 	// 	// Accounts for tests
// 	// 	genesis_account_registry: ROLES
// 	// 		.iter()
// 	// 		.map(|(acc, role)| {
// 	// 			(
// 	// 				*acc,
// 	// 				AccountStruct {
// 	// 					roles: *role
// 	// 				},
// 	// 			)
// 	// 		})
// 	// 		.collect(),
// 	// }
// 	// .assimilate_storage(&mut t)
// 	// .unwrap();

// 	// // t.into()
// 	// let mut ext = sp_io::TestExternalities::new(t);
// 	// ext.execute_with(|| System::set_block_number(1));
// 	// ext
// }
