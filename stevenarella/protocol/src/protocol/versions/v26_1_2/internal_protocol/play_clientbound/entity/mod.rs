use std::io;

use crate::protocol::{packet::Packet, Error};

mod set_entity_data;
mod set_entity_link;
mod set_equipment;
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
        set_equipment::read_set_equipment_clientbound_packet_by_internal_id(internal_id, buf)?
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

    Ok(None)
}
