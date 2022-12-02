use super::*;
big_array! { BigArray; }

pub type IsVCActive = bool;

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