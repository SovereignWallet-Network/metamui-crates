use super::*;

pub fn migrate<T: Config>() -> frame_support::weights::Weight {
    frame_support::debug::RuntimeLogger::init();
    // Storage migrations should use storage versions for safety.
    match PalletVersion::get() {
        StorageVersion::V1_0_0 => {
            
            for (peer_id, did) in Owners::iter() {
                let mut peer_ids = PeerIds::get(did);
                peer_ids.push(peer_id);
                PeerIds::insert(did, peer_ids);
            }

            // Update storage version.
            PalletVersion::put(StorageVersion::V2_0_0);
            // Very inefficient, mostly here for illustration purposes.
            let count = PeerIds::iter().count();

            // Return the weight consumed by the migration.
            T::DbWeight::get().reads_writes(count as Weight + 1, count as Weight + 1)
        }
        _ => {
            frame_support::debug::info!(" >>> Unused migration!");
            0
        }
    }
}
