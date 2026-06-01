use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use crate::protocol::{
    packet::{self, Packet},
    read_nbt_string_component, Direction, Error, Serializable, State, VarInt,
};
use crate::shared::Position;

use super::super::translate_internal_packet_id;

mod entity;
mod scoreboard;
mod sound;
mod update;

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
        packet::play::clientbound::internal_ids::PlaySetDefaultSpawnPositionClientbound => {
            let dimension = String::read_from(buf)?;
            if dimension != "minecraft:overworld" {
                return Err(Error::Err(format!(
                    "unsupported Play set_default_spawn_position dimension {:?}",
                    dimension
                )));
            }
            return Ok(Some(Packet::PlaySetDefaultSpawnPositionClientbound(
                packet::play::clientbound::PlaySetDefaultSpawnPositionClientbound {
                    dimension,
                    location: Position::read_from(buf)?,
                    yaw: buf.read_f32::<BigEndian>()?,
                    pitch: buf.read_f32::<BigEndian>()?,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlaySetPlayerInventoryClientbound => {
            let slot = VarInt::read_from(buf)?;
            read_empty_play_item_stack_marker(buf, "set_player_inventory")?;
            return Ok(Some(Packet::PlaySetPlayerInventoryClientbound(
                packet::play::clientbound::PlaySetPlayerInventoryClientbound { slot, item: None },
            )));
        }
        packet::play::clientbound::internal_ids::PlaySetSubtitleTextClientbound => {
            return Ok(Some(Packet::PlaySetSubtitleTextClientbound(
                packet::play::clientbound::PlaySetSubtitleTextClientbound {
                    text: read_nbt_string_component(buf)?,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlaySetTimeClientbound => {
            let game_time = i64::read_from(buf)?;
            let clock_update_count = VarInt::read_from(buf)?;
            if clock_update_count.0 < 0 {
                return Err(Error::Err(format!(
                    "negative Play set_time clock update count {}",
                    clock_update_count.0
                )));
            }
            if clock_update_count.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play set_time clock update count {}",
                    clock_update_count.0
                )));
            }
            return Ok(Some(Packet::PlaySetTimeClientbound(
                packet::play::clientbound::PlaySetTimeClientbound {
                    game_time,
                    clock_update_count,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlaySetTitleTextClientbound => {
            return Ok(Some(Packet::PlaySetTitleTextClientbound(
                packet::play::clientbound::PlaySetTitleTextClientbound {
                    text: read_nbt_string_component(buf)?,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlayCustomReportDetailsClientbound => {
            let detail_count = VarInt::read_from(buf)?;
            if detail_count.0 < 0 {
                return Err(Error::Err(format!(
                    "negative Play custom_report_details detail count {}",
                    detail_count.0
                )));
            }
            if detail_count.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play custom_report_details map count {}",
                    detail_count.0
                )));
            }
            return Ok(Some(Packet::PlayCustomReportDetailsClientbound(
                packet::play::clientbound::PlayCustomReportDetailsClientbound { detail_count },
            )));
        }
        packet::play::clientbound::internal_ids::PlayServerLinksClientbound => {
            let link_count = VarInt::read_from(buf)?;
            if link_count.0 < 0 {
                return Err(Error::Err(format!(
                    "negative Play server_links link count {}",
                    link_count.0
                )));
            }
            if link_count.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play server_links list count {}",
                    link_count.0
                )));
            }
            return Ok(Some(Packet::PlayServerLinksClientbound(
                packet::play::clientbound::PlayServerLinksClientbound { link_count },
            )));
        }
        packet::play::clientbound::internal_ids::PlayClearDialogClientbound => {
            return Ok(Some(Packet::PlayClearDialogClientbound(
                packet::play::clientbound::PlayClearDialogClientbound {
                    empty: Serializable::read_from(buf)?,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlayShowDialogClientbound => {
            let mut dialog_data = Vec::new();
            buf.read_to_end(&mut dialog_data)?;
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "ShowDialog".to_owned(),
                    data: dialog_data,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlaySystemChatClientbound => {
            let content = read_nbt_string_component(buf)?;
            let overlay = bool::read_from(buf)?;
            return Ok(Some(Packet::PlaySystemChatClientbound(
                packet::play::clientbound::PlaySystemChatClientbound { content, overlay },
            )));
        }
        packet::play::clientbound::internal_ids::PlayTabListClientbound => {
            return Ok(Some(Packet::PlayTabListClientbound(
                packet::play::clientbound::PlayTabListClientbound {
                    header: read_nbt_string_component(buf)?,
                    footer: read_nbt_string_component(buf)?,
                },
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
        packet::play::clientbound::internal_ids::PlayWaypointClientbound => {
            let operation_id = VarInt::read_from(buf)?;
            if operation_id.0 != 1 {
                return Err(Error::Err(format!(
                    "unsupported Play waypoint operation id {}",
                    operation_id.0
                )));
            }
            let mut waypoint_payload = Vec::new();
            buf.read_to_end(&mut waypoint_payload)?;
            let expected_payload = [
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x01, 0x23, 0x11, 0x6d, 0x69, 0x6e, 0x65, 0x63, 0x72, 0x61, 0x66, 0x74, 0x3a,
                0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x00, 0x00,
            ];
            if waypoint_payload != expected_payload {
                return Err(Error::Err(
                    "unsupported Play waypoint payload outside removeWaypoint empty fixture"
                        .to_owned(),
                ));
            }
            return Ok(Some(Packet::PlayWaypointClientbound(
                packet::play::clientbound::PlayWaypointClientbound {
                    operation_id,
                    waypoint_payload,
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
