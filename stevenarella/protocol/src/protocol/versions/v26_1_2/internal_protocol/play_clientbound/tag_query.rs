use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(crate) fn read_tag_query_play_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlayTagQueryClientbound => {
            let transaction_id = VarInt::read_from(buf)?;
            let nbt_tag_type = u8::read_from(buf)?;
            if nbt_tag_type != 10 {
                return Err(Error::Err(format!(
                    "unsupported Play tag_query root NBT tag type {}",
                    nbt_tag_type
                )));
            }
            let mut tag = Vec::new();
            buf.read_to_end(&mut tag)?;
            if tag != [0] {
                return Err(Error::Err(
                    "unsupported non-empty Play tag_query compound payload".to_owned(),
                ));
            }
            Ok(Some(Packet::PlayTagQueryClientbound(
                packet::play::clientbound::PlayTagQueryClientbound {
                    transaction_id,
                    nbt_tag_type,
                    tag,
                },
            )))
        }
        _ => Ok(None),
    }
}
