use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(super) fn read_update_tags_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlayUpdateTagsClientbound => {
            let registry_payload_count = VarInt::read_from(buf)?;
            if registry_payload_count.0 < 0 {
                return Err(Error::Err(format!(
                    "negative Play update_tags registry payload count {}",
                    registry_payload_count.0
                )));
            }
            if registry_payload_count.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play update_tags registry payload count {}",
                    registry_payload_count.0
                )));
            }
            Ok(Some(Packet::PlayUpdateTagsClientbound(
                packet::play::clientbound::PlayUpdateTagsClientbound {
                    registry_payload_count,
                },
            )))
        }
        _ => Ok(None),
    }
}
