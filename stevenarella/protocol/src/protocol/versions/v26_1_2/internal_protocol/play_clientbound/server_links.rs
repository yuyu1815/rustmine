use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(crate) fn read_server_links_play_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlayServerLinksClientbound => {
            let link_count = VarInt::read_from(buf)?;
            if link_count.0 < 0 {
                return Err(Error::Err(format!(
                    "negative Play server_links link count {}",
                    link_count.0
                )));
            }
            if link_count.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play server_links list count {}",
                    link_count.0
                )));
            }
            Ok(Some(Packet::PlayServerLinksClientbound(
                packet::play::clientbound::PlayServerLinksClientbound { link_count },
            )))
        }
        _ => Ok(None),
    }
}
