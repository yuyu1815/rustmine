use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error,
};

pub(crate) fn read_set_cursor_item_play_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlaySetCursorItemClientbound => {
            super::item_stack_marker::read_empty_play_item_stack_marker(buf, "set_cursor_item")?;
            Ok(Some(Packet::PlaySetCursorItemClientbound(
                packet::play::clientbound::PlaySetCursorItemClientbound { item: None },
            )))
        }
        _ => Ok(None),
    }
}
