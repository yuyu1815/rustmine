use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable,
};

pub(super) fn read_set_entity_link_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlaySetEntityLinkClientbound => {
            let source_entity_id = i32::read_from(buf)?;
            let destination_entity_id = i32::read_from(buf)?;
            if source_entity_id != 1 || destination_entity_id != 2 {
                return Err(Error::Err(format!(
                    "unsupported Play set_entity_link fixture source {} destination {}",
                    source_entity_id, destination_entity_id
                )));
            }
            Ok(Some(Packet::PlaySetEntityLinkClientbound(
                packet::play::clientbound::PlaySetEntityLinkClientbound {
                    source_entity_id,
                    destination_entity_id,
                },
            )))
        }
        _ => Ok(None),
    }
}
