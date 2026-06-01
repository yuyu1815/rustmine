use std::io;

use crate::protocol::{
    packet::{self, Packet},
    read_nbt_string_component, Error,
};

pub(super) fn read_tab_list_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlayTabListClientbound => Ok(Some(
            Packet::PlayTabListClientbound(packet::play::clientbound::PlayTabListClientbound {
                header: read_nbt_string_component(buf)?,
                footer: read_nbt_string_component(buf)?,
            }),
        )),
        _ => Ok(None),
    }
}
