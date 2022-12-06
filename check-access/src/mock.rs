use crate as pallet_check_access;
use super::*;

use frame_system as system;
use frame_support::{
	traits::{ GenesisBuild, ConstU16, ConstU32, ConstU64 },
};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use system::EnsureRoot;

pub const FIRST_PALLET_NAME: [u8;32] = [0;32];
pub const FIRST_FUNCTION_NAME: [u8;32] = [1;32];
pub const SECOND_PALLET_NAME: [u8; 32] = [2; 32];
pub const SECOND_FUNCTION_NAME: [u8; 32] = [3; 32];

pub const BLACKLISTED_DID_ONE: [u8; 32] = [10; 32];
pub const BLACKLISTED_DID_TWO: [u8; 32] = [20; 32];

pub const REASON_CODE_ONE: u8 = 1;
pub const REASON_CODE_TWO: u8 = 2;
pub const BLACKLISTING_REASON_ONE: [u8; 32] = [5; 32];
pub const BLACKLISTING_REASON_TWO: [u8; 32] = [6; 32];

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
    CheckAccess: pallet_check_access::{ Pallet, Call, Storage, Event<T> }
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
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
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_check_access::Config for Test {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event = Event;
	/// Trait to resolve Did
  type DidResolution = ();
	/// Sudo Origin
	type CallOrigin = EnsureRoot<Self::AccountId>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut o = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();

		super::GenesisConfig::<Test> {
			initial_extrinsics: vec![InitialExtrinsics {
					pallet_name: FIRST_PALLET_NAME,
					function_name: FIRST_FUNCTION_NAME
				}
			],
			blacklisted_dids: vec![
				(BLACKLISTED_DID_ONE, BLACKLISTING_REASON_ONE),
				(BLACKLISTED_DID_TWO, BLACKLISTING_REASON_TWO),
			],
			blacklisting_reasons: vec![
				(REASON_CODE_ONE, BLACKLISTING_REASON_ONE),
				(REASON_CODE_TWO, BLACKLISTING_REASON_TWO),
			],
			reasons_count: 2,
			phantom: Default::default(),
		}
			.assimilate_storage(&mut o)
			.unwrap();
  o.into()
}
