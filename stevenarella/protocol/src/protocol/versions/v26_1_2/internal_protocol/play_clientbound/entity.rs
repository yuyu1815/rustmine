use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

mod set_entity_data;
mod set_entity_link;
mod set_passengers;
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
        set_entity_link::read_set_entity_link_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        set_entity_data::read_set_entity_data_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        set_passengers::read_set_passengers_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        teleport_entity::read_teleport_entity_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }

    match internal_id {
        packet::play::clientbound::internal_ids::PlaySetEquipmentClientbound => {
            let entity_id = VarInt::read_from(buf)?;
            let equipment_slot = u8::read_from(buf)?;
            if equipment_slot & 0x80 != 0 {
                return Err(Error::Err(format!(
                    "unsupported multi-entry Play set_equipment slot byte {}",
                    equipment_slot
                )));
            }
            super::item_stack_marker::read_empty_play_item_stack_marker(buf, "set_equipment")?;
            Ok(Some(Packet::PlaySetEquipmentClientbound(
                packet::play::clientbound::PlaySetEquipmentClientbound {
                    entity_id,
                    equipment_slot,
                    item: None,
                },
            )))
        }
        _ => Ok(None),
    }
}
