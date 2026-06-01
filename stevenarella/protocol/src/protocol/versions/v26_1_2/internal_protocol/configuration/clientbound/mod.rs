use std::io;

use crate::protocol::{packet::Packet, Direction, Error, State};

use super::super::super::translate_internal_packet_id;

mod code_of_conduct;
mod cookie;
mod custom_payload;
mod custom_report_details;
mod dialog;
mod disconnect_reset_chat;
mod keep_alive_ping;
mod registry_data;
mod resource_pack;
mod select_known_packs;
mod server_links;
mod transfer;
mod update;
mod update_enabled_features;
mod update_tags;

pub(crate) fn read_configuration_clientbound_packet_by_id<R: io::Read>(
    id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    let internal_id =
        translate_internal_packet_id(State::Configuration, Direction::Clientbound, id, true);
    if let Some(packet) =
        cookie::read_cookie_configuration_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        custom_payload::read_custom_payload_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        custom_report_details::read_custom_report_details_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        keep_alive_ping::read_keep_alive_ping_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        update::read_update_configuration_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        resource_pack::read_resource_pack_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        server_links::read_server_links_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        dialog::read_dialog_configuration_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        super::finish_configuration::read_finish_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        code_of_conduct::read_code_of_conduct_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        select_known_packs::read_select_known_packs_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        transfer::read_transfer_configuration_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        registry_data::read_registry_data_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        disconnect_reset_chat::read_disconnect_reset_chat_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }

    Ok(None)
}
