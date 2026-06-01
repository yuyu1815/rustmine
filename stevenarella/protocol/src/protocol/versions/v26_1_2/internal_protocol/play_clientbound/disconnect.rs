use std::io;

use crate::protocol::{
    packet::{self, Packet},
    read_nbt_string_component, Error,
};

pub(crate) fn read_disconnect_play_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::Disconnect => Ok(Some(Packet::Disconnect(
            packet::play::clientbound::Disconnect {
                reason: read_nbt_string_component(buf)?,
            },
        ))),
        _ => Ok(None),
    }
}
