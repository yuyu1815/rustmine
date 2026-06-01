use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(super) fn read_set_entity_data_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlaySetEntityDataClientbound => {
            let entity_id = VarInt::read_from(buf)?;
            let marker = u8::read_from(buf)?;
            if marker != 0xff {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play set_entity_data metadata marker {}",
                    marker
                )));
            }
            Ok(Some(Packet::PlaySetEntityDataClientbound(
                packet::play::clientbound::PlaySetEntityDataClientbound {
                    entity_id,
                    packed_item_count: VarInt(0),
                },
            )))
        }
        _ => Ok(None),
    }
}
