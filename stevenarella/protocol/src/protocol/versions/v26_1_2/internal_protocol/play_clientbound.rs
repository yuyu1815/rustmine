use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use crate::protocol::{
    packet::{self, Packet},
    read_nbt_string_component, Direction, Error, LenPrefixed, Serializable, State, VarInt,
};
use crate::shared::Position;

use super::super::translate_internal_packet_id;

pub(crate) fn read_play_clientbound_packet_by_id<R: io::Read>(
    id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    let internal_id = translate_internal_packet_id(State::Play, Direction::Clientbound, id, true);
    match internal_id {
        packet::play::clientbound::internal_ids::Disconnect => {
            return Ok(Some(Packet::Disconnect(
                packet::play::clientbound::Disconnect {
                    reason: read_nbt_string_component(buf)?,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlaySetDisplayObjectiveClientbound => {
            let slot = VarInt::read_from(buf)?;
            let objective_name = String::read_from(buf)?;
            if !objective_name.is_empty() {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play set_display_objective objective name {:?}",
                    objective_name
                )));
            }
            return Ok(Some(Packet::PlaySetDisplayObjectiveClientbound(
                packet::play::clientbound::PlaySetDisplayObjectiveClientbound {
                    slot,
                    objective_name,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlaySetScoreClientbound => {
            let owner = String::read_from(buf)?;
            let objective_name = String::read_from(buf)?;
            let score = VarInt::read_from(buf)?;
            let display_present = bool::read_from(buf)?;
            if display_present {
                return Err(Error::Err(
                    "unsupported Play set_score optional display Component".to_owned(),
                ));
            }
            let number_format_present = bool::read_from(buf)?;
            if number_format_present {
                return Err(Error::Err(
                    "unsupported Play set_score optional number format".to_owned(),
                ));
            }
            return Ok(Some(Packet::PlaySetScoreClientbound(
                packet::play::clientbound::PlaySetScoreClientbound {
                    owner,
                    objective_name,
                    score,
                    display_present,
                    number_format_present,
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
        packet::play::clientbound::internal_ids::PlaySetEntityDataClientbound => {
            let entity_id = VarInt::read_from(buf)?;
            let marker = u8::read_from(buf)?;
            if marker != 0xff {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play set_entity_data metadata marker {}",
                    marker
                )));
            }
            return Ok(Some(Packet::PlaySetEntityDataClientbound(
                packet::play::clientbound::PlaySetEntityDataClientbound {
                    entity_id,
                    packed_item_count: VarInt(0),
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlaySetEntityLinkClientbound => {
            let source_entity_id = i32::read_from(buf)?;
            let destination_entity_id = i32::read_from(buf)?;
            if source_entity_id != 1 || destination_entity_id != 2 {
                return Err(Error::Err(format!(
                    "unsupported Play set_entity_link fixture source {} destination {}",
                    source_entity_id, destination_entity_id
                )));
            }
            return Ok(Some(Packet::PlaySetEntityLinkClientbound(
                packet::play::clientbound::PlaySetEntityLinkClientbound {
                    source_entity_id,
                    destination_entity_id,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlaySetEquipmentClientbound => {
            let entity_id = VarInt::read_from(buf)?;
            let equipment_slot = u8::read_from(buf)?;
            if equipment_slot & 0x80 != 0 {
                return Err(Error::Err(format!(
                    "unsupported multi-entry Play set_equipment slot byte {}",
                    equipment_slot
                )));
            }
            read_empty_play_item_stack_marker(buf, "set_equipment")?;
            return Ok(Some(Packet::PlaySetEquipmentClientbound(
                packet::play::clientbound::PlaySetEquipmentClientbound {
                    entity_id,
                    equipment_slot,
                    item: None,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlaySetObjectiveClientbound => {
            let objective_name = String::read_from(buf)?;
            let method = i8::read_from(buf)?;
            if method != 1 {
                return Err(Error::Err(format!(
                    "unsupported Play set_objective method {}",
                    method
                )));
            }
            return Ok(Some(Packet::PlaySetObjectiveClientbound(
                packet::play::clientbound::PlaySetObjectiveClientbound {
                    objective_name,
                    method,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlaySetPassengersClientbound => {
            let vehicle_entity_id = VarInt::read_from(buf)?;
            let passenger_entity_ids: LenPrefixed<VarInt, VarInt> = LenPrefixed::read_from(buf)?;
            let passenger_ids: Vec<i32> = passenger_entity_ids.data.iter().map(|id| id.0).collect();
            if vehicle_entity_id.0 != 3 || passenger_ids != [4] {
                return Err(Error::Err(format!(
                    "unsupported Play set_passengers fixture vehicle {} passengers {:?}",
                    vehicle_entity_id.0, passenger_ids
                )));
            }
            return Ok(Some(Packet::PlaySetPassengersClientbound(
                packet::play::clientbound::PlaySetPassengersClientbound {
                    vehicle_entity_id,
                    passenger_entity_ids,
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
        packet::play::clientbound::internal_ids::PlaySetPlayerTeamClientbound => {
            let team_name = String::read_from(buf)?;
            let method = i8::read_from(buf)?;
            if method != 1 {
                return Err(Error::Err(format!(
                    "unsupported Play set_player_team method {}",
                    method
                )));
            }
            return Ok(Some(Packet::PlaySetPlayerTeamClientbound(
                packet::play::clientbound::PlaySetPlayerTeamClientbound { team_name, method },
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
        packet::play::clientbound::internal_ids::PlaySoundEntityClientbound => {
            let sound_holder_id = VarInt::read_from(buf)?;
            if sound_holder_id.0 != 8 {
                return Err(Error::Err(format!(
                    "unsupported Play sound_entity SoundEvent holder id {}",
                    sound_holder_id.0
                )));
            }
            let source = VarInt::read_from(buf)?;
            if source.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported Play sound_entity source id {}",
                    source.0
                )));
            }
            let entity_id = VarInt::read_from(buf)?;
            if entity_id.0 != 1 {
                return Err(Error::Err(format!(
                    "unsupported Play sound_entity fixture entity id {}",
                    entity_id.0
                )));
            }
            let volume = f32::read_from(buf)?;
            if volume != 0.75 {
                return Err(Error::Err(format!(
                    "unsupported Play sound_entity volume {}",
                    volume
                )));
            }
            let pitch = f32::read_from(buf)?;
            if pitch != 1.25 {
                return Err(Error::Err(format!(
                    "unsupported Play sound_entity pitch {}",
                    pitch
                )));
            }
            let seed = i64::read_from(buf)?;
            if seed != 123456789 {
                return Err(Error::Err(format!(
                    "unsupported Play sound_entity seed {}",
                    seed
                )));
            }
            return Ok(Some(Packet::PlaySoundEntityClientbound(
                packet::play::clientbound::PlaySoundEntityClientbound {
                    sound_holder_id,
                    source,
                    entity_id,
                    volume,
                    pitch,
                    seed,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlaySoundClientbound => {
            let sound_holder_id = VarInt::read_from(buf)?;
            if sound_holder_id.0 != 8 {
                return Err(Error::Err(format!(
                    "unsupported Play sound SoundEvent holder id {}",
                    sound_holder_id.0
                )));
            }
            let source = VarInt::read_from(buf)?;
            if source.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported Play sound source id {}",
                    source.0
                )));
            }
            return Ok(Some(Packet::PlaySoundClientbound(
                packet::play::clientbound::PlaySoundClientbound {
                    sound_holder_id,
                    source,
                    x: i32::read_from(buf)?,
                    y: i32::read_from(buf)?,
                    z: i32::read_from(buf)?,
                    volume: f32::read_from(buf)?,
                    pitch: f32::read_from(buf)?,
                    seed: i64::read_from(buf)?,
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
        packet::play::clientbound::internal_ids::PlayTeleportEntityClientbound => {
            let entity_id = VarInt::read_from(buf)?;
            let position_x = buf.read_f64::<BigEndian>()?;
            let position_y = buf.read_f64::<BigEndian>()?;
            let position_z = buf.read_f64::<BigEndian>()?;
            let delta_x = buf.read_f64::<BigEndian>()?;
            let delta_y = buf.read_f64::<BigEndian>()?;
            let delta_z = buf.read_f64::<BigEndian>()?;
            let y_rot = buf.read_f32::<BigEndian>()?;
            let x_rot = buf.read_f32::<BigEndian>()?;
            let relative_mask = buf.read_i32::<BigEndian>()?;
            if relative_mask != 0 {
                return Err(Error::Err(format!(
                    "unsupported Play teleport_entity non-empty relative mask {}",
                    relative_mask
                )));
            }
            return Ok(Some(Packet::PlayTeleportEntityClientbound(
                packet::play::clientbound::PlayTeleportEntityClientbound {
                    entity_id,
                    position_x,
                    position_y,
                    position_z,
                    delta_x,
                    delta_y,
                    delta_z,
                    y_rot,
                    x_rot,
                    relative_mask,
                    on_ground: bool::read_from(buf)?,
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
        packet::play::clientbound::internal_ids::PlayUpdateAttributesClientbound => {
            let entity_id = VarInt::read_from(buf)?;
            let attribute_count = VarInt::read_from(buf)?;
            if attribute_count.0 < 0 {
                return Err(Error::Err(format!(
                    "negative Play update_attributes attribute count {}",
                    attribute_count.0
                )));
            }
            if attribute_count.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play update_attributes attribute count {}",
                    attribute_count.0
                )));
            }
            return Ok(Some(Packet::PlayUpdateAttributesClientbound(
                packet::play::clientbound::PlayUpdateAttributesClientbound {
                    entity_id,
                    attribute_count,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlayUpdateAdvancementsClientbound => {
            let reset = bool::read_from(buf)?;
            let added_count = VarInt::read_from(buf)?;
            let removed_count = VarInt::read_from(buf)?;
            let progress_count = VarInt::read_from(buf)?;
            for (name, count) in [
                ("added", added_count.0),
                ("removed", removed_count.0),
                ("progress", progress_count.0),
            ] {
                if count < 0 {
                    return Err(Error::Err(format!(
                        "negative Play update_advancements {} count {}",
                        name, count
                    )));
                }
                if count != 0 {
                    return Err(Error::Err(format!(
                        "unsupported non-empty Play update_advancements {} count {}",
                        name, count
                    )));
                }
            }
            return Ok(Some(Packet::PlayUpdateAdvancementsClientbound(
                packet::play::clientbound::PlayUpdateAdvancementsClientbound {
                    reset,
                    added_count,
                    removed_count,
                    progress_count,
                    show_advancements: bool::read_from(buf)?,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlayUpdateMobEffectClientbound => {
            let entity_id = VarInt::read_from(buf)?;
            let effect_holder_id = VarInt::read_from(buf)?;
            if effect_holder_id.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported Play update_mob_effect MobEffect holder id {}",
                    effect_holder_id.0
                )));
            }
            return Ok(Some(Packet::PlayUpdateMobEffectClientbound(
                packet::play::clientbound::PlayUpdateMobEffectClientbound {
                    entity_id,
                    effect_holder_id,
                    amplifier: VarInt::read_from(buf)?,
                    duration: VarInt::read_from(buf)?,
                    flags: u8::read_from(buf)?,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlayUpdateRecipesClientbound => {
            let item_set_count = VarInt::read_from(buf)?;
            let stonecutter_recipe_count = VarInt::read_from(buf)?;
            for (name, count) in [
                ("item set", item_set_count.0),
                ("stonecutter recipe", stonecutter_recipe_count.0),
            ] {
                if count < 0 {
                    return Err(Error::Err(format!(
                        "negative Play update_recipes {} count {}",
                        name, count
                    )));
                }
                if count != 0 {
                    return Err(Error::Err(format!(
                        "unsupported non-empty Play update_recipes {} count {}",
                        name, count
                    )));
                }
            }
            return Ok(Some(Packet::PlayUpdateRecipesClientbound(
                packet::play::clientbound::PlayUpdateRecipesClientbound {
                    item_set_count,
                    stonecutter_recipe_count,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlayUpdateTagsClientbound => {
            let registry_payload_count = VarInt::read_from(buf)?;
            if registry_payload_count.0 < 0 {
                return Err(Error::Err(format!(
                    "negative Play update_tags registry payload count {}",
                    registry_payload_count.0
                )));
            }
            if registry_payload_count.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play update_tags registry payload count {}",
                    registry_payload_count.0
                )));
            }
            return Ok(Some(Packet::PlayUpdateTagsClientbound(
                packet::play::clientbound::PlayUpdateTagsClientbound {
                    registry_payload_count,
                },
            )));
        }
        packet::play::clientbound::internal_ids::PlayProjectilePowerClientbound => {
            return Ok(Some(Packet::PlayProjectilePowerClientbound(
                packet::play::clientbound::PlayProjectilePowerClientbound {
                    entity_id: VarInt::read_from(buf)?,
                    acceleration_power: buf.read_f64::<BigEndian>()?,
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
