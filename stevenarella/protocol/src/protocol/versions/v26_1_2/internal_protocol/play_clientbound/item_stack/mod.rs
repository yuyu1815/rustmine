mod item_stack_marker;
mod set_cursor_item;
mod set_player_inventory;

use std::io;

use crate::protocol::Error;

pub(super) use set_cursor_item::read_set_cursor_item_play_clientbound_packet_by_internal_id;
pub(super) use set_player_inventory::read_set_player_inventory_play_clientbound_packet_by_internal_id;

pub(super) fn read_empty_play_item_stack_marker<R: io::Read>(
    buf: &mut R,
    packet_name: &str,
) -> Result<(), Error> {
    item_stack_marker::read_empty_play_item_stack_marker(buf, packet_name)
}
