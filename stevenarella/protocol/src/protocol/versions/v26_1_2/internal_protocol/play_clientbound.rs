use std::io;

use crate::protocol::{
    packet::{self, Packet},
    read_nbt_string_component, Direction, Error, Serializable, State, VarInt,
};

use super::super::translate_internal_packet_id;

mod custom_report_details;
mod dialog;
mod disconnect;
mod entity;
mod scoreboard;
mod server_links;
mod set_cursor_item;
mod set_default_spawn_position;
mod set_player_inventory;
mod set_time;
mod sound;
mod tag_query;
mod text;
mod update;
mod waypoint;

pub(crate) fn read_play_clientbound_packet_by_id<R: io::Read>(
    id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    let internal_id = translate_internal_packet_id(State::Play, Direction::Clientbound, id, true);
    if let Some(packet) =
        scoreboard::read_scoreboard_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) = sound::read_sound_clientbound_packet_by_internal_id(internal_id, buf)? {
        return Ok(Some(packet));
    }
    if let Some(packet) = update::read_update_clientbound_packet_by_internal_id(internal_id, buf)? {
        return Ok(Some(packet));
    }
    if let Some(packet) = entity::read_entity_clientbound_packet_by_internal_id(internal_id, buf)? {
        return Ok(Some(packet));
    }
    if let Some(packet) = text::read_text_clientbound_packet_by_internal_id(internal_id, buf)? {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        dialog::read_dialog_play_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        waypoint::read_waypoint_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        set_time::read_set_time_play_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        custom_report_details::read_custom_report_details_play_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        server_links::read_server_links_play_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        set_default_spawn_position::read_set_default_spawn_position_play_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        set_cursor_item::read_set_cursor_item_play_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        disconnect::read_disconnect_play_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        set_player_inventory::read_set_player_inventory_play_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        tag_query::read_tag_query_play_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }

    match internal_id {
        packet::play::clientbound::internal_ids::PlayTestInstanceBlockStatusClientbound => {
            let status = read_nbt_string_component(buf)?;
            let size_present = bool::read_from(buf)?;
            if size_present {
                return Err(Error::Err(
                    "unsupported Play test_instance_block_status present size".to_owned(),
                ));
            }
            return Ok(Some(Packet::PlayTestInstanceBlockStatusClientbound(
                packet::play::clientbound::PlayTestInstanceBlockStatusClientbound {
                    status,
                    size_present,
                },
            )));
        }
        _ => Ok(None),
    }
}

fn read_empty_play_item_stack_marker<R: io::Read>(
    buf: &mut R,
    packet_name: &str,
) -> Result<(), Error> {
    let count = VarInt::read_from(buf)?;
    if count.0 != 0 {
        return Err(Error::Err(format!(
            "unsupported non-empty Play {} ItemStack count {}",
            packet_name, count.0
        )));
    }
    Ok(())
}
