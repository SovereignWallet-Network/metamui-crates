use codec::{Decode, Encode};
use sp_runtime::{
  RuntimeDebug,
};


/// Utility type for managing upgrades/migrations.
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq)]
pub enum StorageVersion {
    V1_0_0,
    V2_0_0,
}