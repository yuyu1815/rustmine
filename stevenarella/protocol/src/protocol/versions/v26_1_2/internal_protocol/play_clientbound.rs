use std::io;

use crate::protocol::{
    packet::{self, Packet},
    read_nbt_string_component, Direction, Error, Serializable, State, VarInt,
};

use super::super::translate_internal_packet_id;

mod custom_report_details;
mod dialog;
mod entity;
mod scoreboard;
mod server_links;
mod set_default_spawn_position;
mod set_time;
mod sound;
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

    match internal_id {
        packet::play::clientbound::internal_ids::Disconnect => {
            return Ok(Some(Packet::Disconnect(
                packet::play::clientbound::Disconnect {
                    reason: read_nbt_string_component(buf)?,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlaySetCursorItemClientbound => {
            read_empty_play_item_stack_marker(buf, "set_cursor_item")?;
            return Ok(Some(Packet::PlaySetCursorItemClientbound(
                packet::play::clientbound::PlaySetCursorItemClientbound { item: None },
            )));
        }
        packet::play::clientbound::internal_ids::PlaySetPlayerInventoryClientbound => {
            let slot = VarInt::read_from(buf)?;
            read_empty_play_item_stack_marker(buf, "set_player_inventory")?;
            return Ok(Some(Packet::PlaySetPlayerInventoryClientbound(
                packet::play::clientbound::PlaySetPlayerInventoryClientbound { slot, item: None },
            )));
        }
        packet::play::clientbound::internal_ids::PlayTagQueryClientbound => {
            let transaction_id = VarInt::read_from(buf)?;
            let nbt_tag_type = u8::read_from(buf)?;
            if nbt_tag_type != 10 {
                return Err(Error::Err(format!(
                    "unsupported Play tag_query root NBT tag type {}",
                    nbt_tag_type
                )));
            }
            let mut tag = Vec::new();
            buf.read_to_end(&mut tag)?;
            if tag != [0] {
                return Err(Error::Err(
                    "unsupported non-empty Play tag_query compound payload".to_owned(),
                ));
            }
            return Ok(Some(Packet::PlayTagQueryClientbound(
                packet::play::clientbound::PlayTagQueryClientbound {
                    transaction_id,
                    nbt_tag_type,
                    tag,
                },
            )));
        }
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
