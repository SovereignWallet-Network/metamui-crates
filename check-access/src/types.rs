use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

pub type PalletName = [u8; 32];
pub type FunctionName = [u8; 32];

#[derive(Debug, Clone, Decode, Encode, TypeInfo, Eq, PartialEq, MaxEncodedLen)]
pub struct ExtrinsicsStruct { 
	pub pallet_name: PalletName,
	pub function_name: FunctionName,
}