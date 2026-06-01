use crate::item;
use crate::protocol::{Conn, Error, VarInt, Version};

pub enum InventoryOperation {
    LeftClick,
    RightClick,
    LeftClickOutside,
    RightClickOutside,
    ShiftLeftClick,
    ShiftRightClick,
    NumberKey(u8),
    OffHandSwap,
    MiddleClick,
    DropItem,
    DropStack,
    StartLeftDrag,
    StartRightDrag,
    StartMiddleDrag,
    AddLeftDragSlot,
    AddRightDragSlot,
    AddMiddleDragSlot,
    EndLeftDrag,
    EndRightDrag,
    EndMiddleDrag,
    DoubleClick,
}

impl InventoryOperation {
    fn to_mode_and_button_ids(&self) -> (u8, u8) {
        match *self {
            InventoryOperation::LeftClick => (0, 0),
            InventoryOperation::RightClick => (0, 1),
            InventoryOperation::LeftClickOutside => (0, 0),
            InventoryOperation::RightClickOutside => (0, 1),
            InventoryOperation::ShiftLeftClick => (1, 0),
            InventoryOperation::ShiftRightClick => (1, 1),
            InventoryOperation::NumberKey(n) => (2, n - 1),
            InventoryOperation::OffHandSwap => (2, 40),
            InventoryOperation::MiddleClick => (3, 2),
            InventoryOperation::DropItem => (4, 0),
            InventoryOperation::DropStack => (4, 1),
            InventoryOperation::StartLeftDrag => (5, 0),
            InventoryOperation::StartRightDrag => (5, 4),
            InventoryOperation::StartMiddleDrag => (5, 8),
            InventoryOperation::AddLeftDragSlot => (5, 1),
            InventoryOperation::AddRightDragSlot => (5, 5),
            InventoryOperation::AddMiddleDragSlot => (5, 9),
            InventoryOperation::EndLeftDrag => (5, 2),
            InventoryOperation::EndRightDrag => (5, 6),
            InventoryOperation::EndMiddleDrag => (5, 10),
            InventoryOperation::DoubleClick => (6, 0),
        }
    }
}

pub fn send_click_container(
    conn: &mut Conn,
    id: u8,
    slot: i16,
    operation: InventoryOperation,
    action_number: u16,
    clicked_item: Option<item::Stack>,
) -> Result<(), Error> {
    let (mode, button) = operation.to_mode_and_button_ids();
    let version = conn.get_version();
    if version < Version::V1_9 {
        conn.write_packet(super::play::serverbound::ClickWindow_u8 {
            id,
            slot,
            button,
            action_number,
            mode,
            clicked_item,
        })
    } else {
        conn.write_packet(super::play::serverbound::ClickWindow {
            id,
            slot,
            button,
            action_number,
            mode: VarInt(mode as i32),
            clicked_item,
        })
    }
}

pub fn send_close_window(conn: &mut Conn, id: u8) -> Result<(), Error> {
    conn.write_packet(super::play::serverbound::CloseWindow { id })
}
