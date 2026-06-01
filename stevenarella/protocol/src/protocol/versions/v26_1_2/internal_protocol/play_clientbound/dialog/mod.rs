use std::io;

use crate::protocol::{packet::Packet, Error};

mod clear_dialog;
mod show_dialog;

pub(crate) fn read_dialog_play_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    if let Some(packet) =
        clear_dialog::read_clear_dialog_play_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        show_dialog::read_show_dialog_play_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }

    Ok(None)
}
