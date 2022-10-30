use crate as pallet_did;
use metamui_primitives::traits::IsValidator;
use pallet_vc;
use metamui_primitives::types::{ VCType, CompanyName, RegistrationNumber, VC };
use metamui_primitives::VCid;
use crate::types::*;
use frame_support::{
	traits::{ GenesisBuild, ConstU16, ConstU32, ConstU64, OnInitialize, OnFinalize },
};

use codec::{ Encode, Decode };
use sp_core::{sr25519, Pair, H256, Public};
use frame_system as system;
use sp_runtime::{
	testing::Header,
	traits::{ BlakeTwo256, IdentityLookup, Hash },
};
use system::EnsureSigned;

pub const VALIDATOR_DID: [u8; 32] = *b"did:ssid:swn\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
pub const VALIDATOR_ACCOUNT: u64 = 2077282123132384724;
pub const VALIDATOR_SEED: [u8; 32] = [
  229, 190, 154, 80, 146, 184, 27, 202, 100, 190, 129, 210, 18, 231, 242, 249, 235, 161, 131,
  187, 122, 144, 149, 79, 123, 118, 54, 31, 110, 219, 92, 10,
];
pub const NON_VALIDATOR_ACCOUNT: u64 = 2;

// pub const PRIVATE_DID: [u8; 32] = *b"did:ssid:Johny\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// pub const PRIVATE_PUBKEY: sr25519::Public = sr25519::Public([1; 32]);

// pub const PUBLIC_DID: [u8; 32] = *b"did:ssid:Toony\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
// pub const PUBLIC_PUBKEY: sr25519::Public = sr25519::Public([2; 32]);

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
		Did: pallet_did::{Pallet, Call, Storage, Event<T>, Config<T>},
		VcPallet: pallet_vc::{Pallet, Call, Storage, Event<T>, Config<T>},
	}
);

pub struct IsValidatorImplemented;
impl IsValidator for IsValidatorImplemented {

	fn is_validator(_who: &[u8; 32]) -> bool {
		false
	}

  /// Check if given did has global permission level
  fn is_validator_global(_did: &[u8; 32]) -> bool {
    false
  }

	fn get_region(_did: [u8; 32]) -> Region {
		vec![]
  }

	/// Check if given did has permission in given region
  fn has_regional_permission(_did: &[u8; 32], _region: Region) -> bool {
		true
	}
}

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

impl pallet_did::Config for Test {
	type Event = Event;
	type ValidatorOrigin = EnsureSigned<Self::AccountId>;
	type MaxKeyChanges = ConstU32<16>;
	type VCResolution = VcPallet;
	type OnDidUpdate = ();
}

impl pallet_vc::Config for Test {
	type Event = Event;
	type ApproveOrigin = EnsureSigned<Self::AccountId>;
	type IsCouncilMember = ();
	type IsValidator = IsValidatorImplemented;
	type DidResolution = Did;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut o = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();
	
	super::GenesisConfig::<Test> { 
		initial_dids: vec![DIdentity::Private(
			PrivateDid {
				identifier: VALIDATOR_DID,
				public_key: sr25519::Pair::from_seed(&VALIDATOR_SEED).public(),
				metadata: Default::default(),
			}
		)],
		phantom: Default::default(),
	}
		.assimilate_storage(&mut o)
		.unwrap();
	o.into()
}
	
pub fn get_public_did_vc(identifier: [u8; 32], public_key: PublicKey) -> [u8; 128]{
	let public_key = public_key;
	let did = identifier;
	let registration_number: RegistrationNumber = Default::default();
	let company_name: CompanyName = Default::default();
	let did_vc= PublicDidVC{
		public_key,
		registration_number,
		company_name,
		did
	};
	convert_to_array::<128>(did_vc.encode())
}

pub fn get_private_did_vc(identifier: [u8; 32], public_key: PublicKey) -> [u8; 128]{
	let public_key = public_key;
	let did = identifier;
	let did_vc = PrivateDidVC{
		public_key,
		did
	};
	convert_to_array::<128>(did_vc.encode())
}

pub fn get_vc_id_and_hex(did_vc_bytes: [u8; 128], vc_type: VCType) -> ([u8; 32], Vec<u8>) {
	let pair: sr25519::Pair = sr25519::Pair::from_seed(&VALIDATOR_SEED);
	let owner = VALIDATOR_DID;
	let issuers = vec![VALIDATOR_DID];
	let hash = BlakeTwo256::hash_of(&(&vc_type, &did_vc_bytes, &owner, &issuers));
	let signature = pair.sign(hash.as_ref());
	let vc_struct = VC {
		hash,
		owner,
		issuers,
		signatures: vec![signature],
		is_vc_used: false,
		is_vc_active: true,
		vc_type,
		vc_property: did_vc_bytes,
	};
	let vc_id: VCid = *BlakeTwo256::hash_of(&vc_struct).as_fixed_bytes();
  (vc_id, vc_struct.encode())
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

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
	}
}
