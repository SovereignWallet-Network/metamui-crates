use super::pallet::*;
use codec::{ Decode };
use sp_runtime::{ DispatchError };
use metamui_primitives::{ VCid, traits::VCResolve, types::VC };

impl<T: Config> VCResolve<T::Hash> for Pallet<T> {
  /// Decoding VC from encoded bytes
  fn decode_vc<E: Decode>(vc_bytes: &[u8]) -> Result<E, DispatchError> {
    Self::decode_vc::<E>(vc_bytes)
  }   

  fn get_vc(vc_id: &VCid) -> Option<VC<T::Hash>> {
    VCs::<T>::get(vc_id)
  }

  fn is_vc_used(vc_id: &VCid) -> bool {
    match VCs::<T>::get(vc_id) {
      Some(vc) => vc.is_vc_used,
      None => false
    }
  }

  fn set_is_vc_used(vc_id: &VCid, is_vc_used: bool) {
    Self::set_is_used_flag(*vc_id, Some(is_vc_used));
  }
}
