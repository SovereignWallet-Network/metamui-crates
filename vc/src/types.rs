use super::*;
big_array! { BigArray; }

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TokenVC {
    pub token_name: [u8; 16],
    pub reservable_balance: u128,
    pub decimal: u8,
    pub currency_code: [u8; 8],
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericVC {
    #[cfg_attr(feature = "serde", serde(with = "BigArray"))]
    pub cid: [u8; 64]
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitialVCs {
    pub identifier: Did,
    pub public_key: PublicKey,
    pub vcs: Vec<VCHash>,
}

/// Utility type for managing upgrades/migrations.
#[derive(codec::Encode, codec::Decode, Clone, frame_support::RuntimeDebug, PartialEq)]
pub enum VCPalletVersion {
	V1_0_0,
	V2_0_0,
}
