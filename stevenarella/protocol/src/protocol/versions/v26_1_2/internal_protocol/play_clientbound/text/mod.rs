use std::io;

use crate::protocol::{packet::Packet, Error};

mod subtitle_text;
mod system_chat;
mod tab_list;
mod title_text;

pub(crate) fn read_text_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    if let Some(packet) =
        subtitle_text::read_set_subtitle_text_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        title_text::read_set_title_text_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        system_chat::read_system_chat_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        tab_list::read_tab_list_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }

    Ok(None)
}
