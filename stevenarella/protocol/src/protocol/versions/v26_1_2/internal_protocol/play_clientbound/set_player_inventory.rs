use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(crate) fn read_set_player_inventory_play_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlaySetPlayerInventoryClientbound => {
            let slot = VarInt::read_from(buf)?;
            super::read_empty_play_item_stack_marker(buf, "set_player_inventory")?;
            Ok(Some(Packet::PlaySetPlayerInventoryClientbound(
                packet::play::clientbound::PlaySetPlayerInventoryClientbound { slot, item: None },
            )))
        }
        _ => Ok(None),
    }
}
