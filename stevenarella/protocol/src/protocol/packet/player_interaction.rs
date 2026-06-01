use cgmath::{Vector3, Zero};

use crate::item::Stack;
use crate::protocol::{packet, Conn, Error, VarInt};
use crate::shared::{Position, Version};

use super::{DigType, Hand};

pub fn send_arm_swing(conn: &mut Conn, hand: Hand) -> Result<(), Error> {
    let version = conn.get_version();
    if version < Version::V1_8 {
        conn.write_packet(packet::play::serverbound::ArmSwing_Handsfree_ID {
            entity_id: 0, // TODO: Check these values!
            animation: 0,
        })
    } else if version < Version::V1_9 {
        conn.write_packet(packet::play::serverbound::ArmSwing_Handsfree { empty: () })
    } else {
        conn.write_packet(packet::play::serverbound::ArmSwing {
            hand: VarInt(hand.ordinal()),
        })
    }
}

pub fn send_digging(
    conn: &mut Conn,
    status: DigType,
    pos: Position,
    face_index: u8,
) -> Result<(), Error> {
    let version = conn.get_version();
    if version < Version::V1_8 {
        conn.write_packet(packet::play::serverbound::PlayerDigging_u8_u8y {
            status: status.ordinal() as u8,
            x: pos.x,
            y: pos.y as u8,
            z: pos.z,
            face: face_index,
        })
    } else if version < Version::V1_9 {
        conn.write_packet(packet::play::serverbound::PlayerDigging_u8 {
            status: status.ordinal() as u8,
            location: pos,
            face: face_index,
        })
    } else {
        conn.write_packet(packet::play::serverbound::PlayerDigging {
            status: VarInt(status.ordinal()),
            location: pos,
            face: face_index,
        })
    }
}

pub fn send_use_item(
    conn: &mut Conn,
    hand: Hand,
    cursor_position: Option<Vector3<f64>>,
    item: Option<Stack>,
) -> Result<(), Error> {
    let version = conn.get_version();
    if version <= Version::V1_8 {
        let cursor_position = cursor_position.unwrap_or(Vector3::zero());
        conn.write_packet(packet::play::serverbound::PlayerBlockPlacement_u8_Item {
            location: Position::new(-1, -1, -1),
            face: -1,
            hand: item,
            cursor_x: (cursor_position.x * 16.0) as u8,
            cursor_y: (cursor_position.y * 16.0) as u8,
            cursor_z: (cursor_position.z * 16.0) as u8,
        })
    } else {
        conn.write_packet(packet::play::serverbound::UseItem {
            hand: VarInt(hand.ordinal()),
        })
    }
}

pub fn send_drop_item(conn: &mut Conn, whole_stack: bool) -> Result<(), Error> {
    send_digging(
        conn,
        if whole_stack {
            DigType::DropAllItems
        } else {
            DigType::DropItem
        },
        Position::new(0, 0, 0),
        0,
    )
}

pub fn send_swap_item_in_hand(conn: &mut Conn) -> Result<(), Error> {
    send_digging(conn, DigType::SwapItemInHand, Position::new(0, 0, 0), 0)
}

/// shoot an arrow or finish eating
pub fn send_release_use_item(conn: &mut Conn) -> Result<(), Error> {
    send_digging(conn, DigType::ReleaseUseItem, Position::new(0, 0, 0), 255)
}

pub fn send_block_place(
    conn: &mut Conn,
    pos: Position,
    face: i8,
    cursor_position: Vector3<f64>,
    hand: Hand,
    item: Option<Stack>,
) -> Result<(), Error> {
    let version = conn.get_version();
    if version >= Version::V1_14 {
        conn.write_packet(
            packet::play::serverbound::PlayerBlockPlacement_insideblock {
                location: pos,
                face: VarInt(face as i32),
                hand: VarInt(hand.ordinal()),
                cursor_x: cursor_position.x as f32,
                cursor_y: cursor_position.y as f32,
                cursor_z: cursor_position.z as f32,
                inside_block: false,
            },
        )
    } else if version >= Version::V1_11 {
        conn.write_packet(packet::play::serverbound::PlayerBlockPlacement_f32 {
            location: pos,
            face: VarInt(face as i32),
            hand: VarInt(hand.ordinal()),
            cursor_x: cursor_position.x as f32,
            cursor_y: cursor_position.y as f32,
            cursor_z: cursor_position.z as f32,
        })
    } else if version >= Version::V1_9 {
        // TODO: for protocol version >= 49
        conn.write_packet(packet::play::serverbound::PlayerBlockPlacement_u8 {
            location: pos,
            face: VarInt(face as i32),
            hand: VarInt(hand.ordinal()),
            cursor_x: (cursor_position.x * 16.0) as u8,
            cursor_y: (cursor_position.y * 16.0) as u8,
            cursor_z: (cursor_position.z * 16.0) as u8,
        })
    } else if version >= Version::V1_8 {
        conn.write_packet(packet::play::serverbound::PlayerBlockPlacement_u8_Item {
            location: pos,
            face,
            hand: item,
            cursor_x: (cursor_position.x * 16.0) as u8,
            cursor_y: (cursor_position.y * 16.0) as u8,
            cursor_z: (cursor_position.z * 16.0) as u8,
        })
    } else {
        conn.write_packet(
            packet::play::serverbound::PlayerBlockPlacement_u8_Item_u8y {
                x: pos.x,
                y: pos.y as u8,
                z: pos.x,
                face,
                hand: item,
                cursor_x: (cursor_position.x * 16.0) as u8,
                cursor_y: (cursor_position.y * 16.0) as u8,
                cursor_z: (cursor_position.z * 16.0) as u8,
            },
        )
    }
}
