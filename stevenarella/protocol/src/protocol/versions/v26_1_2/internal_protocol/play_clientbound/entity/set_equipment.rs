use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(super) fn read_set_equipment_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
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
            super::super::item_stack::read_empty_play_item_stack_marker(buf, "set_equipment")?;
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
