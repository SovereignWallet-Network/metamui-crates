use super::*;
big_array! { BigArray; }

pub type IsVCActive = bool;

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
    pub vc_id: VCid,
    pub vc_hex: VCHex,
}