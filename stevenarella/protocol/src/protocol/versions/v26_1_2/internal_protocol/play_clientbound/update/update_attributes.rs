use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(super) fn read_update_attributes_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlayUpdateAttributesClientbound => {
            let entity_id = VarInt::read_from(buf)?;
            let attribute_count = VarInt::read_from(buf)?;
            if attribute_count.0 < 0 {
                return Err(Error::Err(format!(
                    "negative Play update_attributes attribute count {}",
                    attribute_count.0
                )));
            }
            if attribute_count.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play update_attributes attribute count {}",
                    attribute_count.0
                )));
            }
            Ok(Some(Packet::PlayUpdateAttributesClientbound(
                packet::play::clientbound::PlayUpdateAttributesClientbound {
                    entity_id,
                    attribute_count,
                },
            )))
        }
        _ => Ok(None),
    }
}
