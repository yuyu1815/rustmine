use std::io;

use crate::protocol::{packet::Packet, Direction, Error, State};

use super::super::super::translate_internal_packet_id;

mod accept_code_of_conduct;
mod client_information;
mod cookie_response;
mod custom_click_action;
mod custom_payload;
mod keep_alive;
mod pong;
mod resource_pack;
mod select_known_packs;

pub(crate) fn read_configuration_serverbound_packet_by_id<R: io::Read>(
    id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    let internal_id =
        translate_internal_packet_id(State::Configuration, Direction::Serverbound, id, true);
    if let Some(packet) =
        custom_click_action::read_custom_click_action_configuration_serverbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        super::finish_configuration::read_finish_configuration_serverbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        accept_code_of_conduct::read_accept_code_of_conduct_configuration_serverbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        keep_alive::read_keep_alive_configuration_serverbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        custom_payload::read_custom_payload_configuration_serverbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        pong::read_pong_configuration_serverbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        resource_pack::read_resource_pack_configuration_serverbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        select_known_packs::read_select_known_packs_configuration_serverbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        cookie_response::read_cookie_response_configuration_serverbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        client_information::read_client_information_configuration_serverbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }

    Ok(None)
}
