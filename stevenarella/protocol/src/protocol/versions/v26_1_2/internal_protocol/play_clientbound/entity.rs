use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, LenPrefixed, Serializable, VarInt,
};

use super::read_empty_play_item_stack_marker;

mod teleport_entity;

pub(crate) fn read_entity_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    if let Some(packet) =
        super::projectile_power::read_projectile_power_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        teleport_entity::read_teleport_entity_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }

    match internal_id {
        packet::play::clientbound::internal_ids::PlaySetEntityDataClientbound => {
            let entity_id = VarInt::read_from(buf)?;
            let marker = u8::read_from(buf)?;
            if marker != 0xff {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play set_entity_data metadata marker {}",
                    marker
                )));
            }
            Ok(Some(Packet::PlaySetEntityDataClientbound(
                packet::play::clientbound::PlaySetEntityDataClientbound {
                    entity_id,
                    packed_item_count: VarInt(0),
                },
            )))
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
            Ok(Some(Packet::PlaySetEntityLinkClientbound(
                packet::play::clientbound::PlaySetEntityLinkClientbound {
                    source_entity_id,
                    destination_entity_id,
                },
            )))
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
            Ok(Some(Packet::PlaySetEquipmentClientbound(
                packet::play::clientbound::PlaySetEquipmentClientbound {
                    entity_id,
                    equipment_slot,
                    item: None,
                },
            )))
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
            Ok(Some(Packet::PlaySetPassengersClientbound(
                packet::play::clientbound::PlaySetPassengersClientbound {
                    vehicle_entity_id,
                    passenger_entity_ids,
                },
            )))
        }
        _ => Ok(None),
    }
}
