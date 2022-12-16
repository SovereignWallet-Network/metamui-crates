use crate::{self as verified_credentials, Config};
use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{ConstU32, Everything, GenesisBuild},
};
use frame_system::{EnsureSigned, EnsureSignedBy};
use pallet_did::types::{DIdentity, PrivateDid};
use sp_core::{sr25519, Pair, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use sp_std::convert::TryInto;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type BlockNumber = u32;


ord_parameter_types! {
	pub const ValidAccount: u64 = BOB_ACCOUNT_ID;
}

const MILLISECS_PER_BLOCK: u64 = 5000;
const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
const HOURS: BlockNumber = MINUTES * 60;
const DAYS: BlockNumber = HOURS * 24;
// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		VC: verified_credentials::{Pallet, Call, Storage, Event<T>},
		ValidatorSet: pallet_validator_set::{Pallet, Call, Storage, Event<T>, Config<T>},
		Did: pallet_did::{Pallet, Call, Storage, Config<T>, Event<T>},
		Council: pallet_collective::<Instance1>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>},
		ValidatorCommittee: pallet_validator_collective::<Instance1>::{Pallet, Call, Storage, Origin<T>, Event<T>, Config<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
	pub const CouncilMotionDuration: BlockNumber = 5 * MINUTES;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
	pub const MaxValidators : u32 = 20;
}

impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
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
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl Config for Test {
	type Event = Event;
	type ApproveOrigin = EnsureSignedBy<ValidAccount, u64>;
	type IsCouncilMember = Council;
	type IsValidator = ValidatorCommittee;
	type DidResolution = Did;
}

ord_parameter_types! {
	pub const One: u64 = 1;
	pub const Two: u64 = 2;
	pub const Three: u64 = 3;
	pub const Four: u64 = 4;
	pub const Five: u64 = 5;
	pub const Six: u64 = 6;
}

impl pallet_validator_set::Config for Test {
	type Event = Event;
	type AddOrigin = EnsureSignedBy<One, u64>;
	type RemoveOrigin = EnsureSignedBy<Two, u64>;
	type SwapOrigin = EnsureSignedBy<Three, u64>;
	type ResetOrigin = EnsureSignedBy<Four, u64>;
	type PrimeOrigin = EnsureSignedBy<Five, u64>;
	type MembershipInitialized = ValidatorCommittee;
	type MembershipChanged = ValidatorCommittee;
	type MaxMembers = MaxValidators;
	type DidResolution = Did;
	type WeightInfo = ();
}

impl pallet_did::Config for Test {
	type Event = Event;
	type ValidatorOrigin = EnsureSigned<Self::AccountId>;
	type MaxKeyChanges = ConstU32<16>;
	type OnDidUpdate = ();
	type VCResolution = VC;
}

pub type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Test {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type DidResolution = Did;
	type WeightInfo = ();
}

parameter_types! {
	pub const TechnicalMotionDuration: BlockNumber = 7 * DAYS;
	pub const TechnicalMaxProposals: u32 = 100;
	pub const TechnicalMaxMembers: u32 = 100;
}

pub type ValidatorCollective = pallet_validator_collective::Instance1;
impl pallet_validator_collective::Config<ValidatorCollective> for Test {
	type Event = Event;
	type Origin = Origin;
	type Proposal = Call;
	type DidResolution = Did;
	type CallOrigin = EnsureSignedBy<Six, u64>;
	type MaxMembers = TechnicalMaxMembers;
	type WeightInfo = ();
}

pub const VALIDATOR_ACCOUNT: u64 = 0;
pub const VALIDATOR_DID: [u8; 32] = *b"did:ssid:Alice\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const VALIDATOR_PUBKEY: sr25519::Public = sr25519::Public([0; 32]);
pub const NON_VALIDATOR_ACCOUNT: u64 = 2;
pub const ALICE: metamui_primitives::Did = *b"did:ssid:swn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const BOB: metamui_primitives::Did = *b"did:ssid:bob\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const DAVE: metamui_primitives::Did = *b"did:ssid:dave\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const EVE: metamui_primitives::Did = *b"did:ssid:eve\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const DAVE_ACCOUNT_ID: u64 = 13620103657161844528;
pub const BOB_ACCOUNT_ID: u64 = 7166219960988249998;
pub const BOB_SEED: [u8; 32] = [
	57, 143, 12, 40, 249, 136, 133, 224, 70, 51, 61, 74, 65, 193, 156, 238, 76, 55, 54, 138, 152,
	50, 198, 80, 47, 108, 253, 24, 46, 42, 239, 137,
];
pub const DAVE_SEED: [u8; 32] = [
	134, 128, 32, 174, 6, 135, 221, 167, 213, 117, 101, 9, 58, 105, 9, 2, 17, 68, 152, 69, 167,
	225, 20, 83, 97, 40, 0, 182, 99, 48, 114, 70,
];
pub const EVE_SEED: [u8; 32] = [
	120, 106, 208, 226, 223, 69, 111, 228, 61, 209, 249, 30, 188, 162, 46, 35, 91, 193, 98, 224,
	187, 141, 83, 198, 51, 232, 200, 91, 42, 246, 139, 122,
];

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut o = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	pallet_validator_set::GenesisConfig::<Test> {
		members: frame_support::bounded_vec![BOB, DAVE],
		phantom: Default::default(),
	}
	.assimilate_storage(&mut o)
	.unwrap();

	pallet_did::GenesisConfig::<Test> {
		initial_dids: vec![
			DIdentity::Private(PrivateDid {
				identifier: BOB,
				public_key: sr25519::Pair::from_seed(&BOB_SEED).public(),
				metadata: Default::default(),
			}),
			DIdentity::Private(PrivateDid {
				identifier: DAVE,
				public_key: sr25519::Pair::from_seed(&DAVE_SEED).public(),
				metadata: Default::default(),
			}),
			DIdentity::Private(PrivateDid {
				identifier: VALIDATOR_DID,
				public_key: VALIDATOR_PUBKEY,
				metadata: Default::default(),
			}),
			DIdentity::Private(PrivateDid {
				identifier: EVE,
				public_key: sr25519::Pair::from_seed(&EVE_SEED).public(),
				metadata: Default::default(),
			}),
		],

		phantom: Default::default(),
	}
	.assimilate_storage(&mut o)
	.unwrap();

	pallet_collective::GenesisConfig::<Test, pallet_collective::Instance1> {
		members: vec![ALICE, BOB, DAVE],
		phantom: Default::default(),
	}
	.assimilate_storage(&mut o)
	.unwrap();
	o.into()
}


pub fn convert_to_array<const N: usize>(mut v: Vec<u8>) -> [u8; N] {
	if v.len() != N {
		for _ in v.len()..N {
			v.push(0);
		}
	}
	v.try_into().unwrap_or_else(|v: Vec<u8>| {
		panic!("Expected a Vec of length {} but it was {}", N, v.len())
	})
}
