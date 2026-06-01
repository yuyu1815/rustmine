use std::io;

use crate::protocol::{
    packet::{self, Packet},
    read_nbt_string_component, Error,
};

pub(super) fn read_set_title_text_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlaySetTitleTextClientbound => {
            Ok(Some(Packet::PlaySetTitleTextClientbound(
                packet::play::clientbound::PlaySetTitleTextClientbound {
                    text: read_nbt_string_component(buf)?,
                },
            )))
        }
        _ => Ok(None),
    }
}
