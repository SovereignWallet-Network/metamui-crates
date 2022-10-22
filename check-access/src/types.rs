pub type PalletName = [u8; 32];
pub type FunctionName = [u8; 32];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitialExtrinsics {
  pub pallet_name: PalletName, 
  pub function_name: FunctionName
}