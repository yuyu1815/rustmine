use cgmath::Vector3;
use std::f32::consts::PI;

use crate::protocol::{Conn, Error};
use crate::shared::Version;

pub fn send_position_look(
    conn: &mut Conn,
    position: &Vector3<f64>,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
) -> Result<(), Error> {
    let version = conn.get_version();
    if version >= Version::V1_8 {
        conn.write_packet(
            crate::protocol::packet::play::serverbound::PlayerPositionLook {
                x: position.x,
                y: position.y,
                z: position.z,
                yaw: -yaw * (180.0 / PI),
                pitch: (-pitch * (180.0 / PI) + 180.0).min(90.0), // used to make sure, that we don't send impossible pitch values
                on_ground,
            },
        )
    } else {
        conn.write_packet(
            crate::protocol::packet::play::serverbound::PlayerPositionLook_HeadY {
                x: position.x,
                feet_y: position.y,
                head_y: position.y + 1.62,
                z: position.z,
                yaw: -yaw * (180.0 / PI),
                pitch: (-pitch * (180.0 / PI) + 180.0).min(90.0), // used to make sure, that we don't send impossible pitch values
                on_ground,
            },
        )
    }
}

pub fn send_position(
    conn: &mut Conn,
    position: &Vector3<f64>,
    on_ground: bool,
) -> Result<(), Error> {
    let version = conn.get_version();
    if version >= Version::V1_8 {
        conn.write_packet(crate::protocol::packet::play::serverbound::PlayerPosition {
            x: position.x,
            y: position.y,
            z: position.z,
            on_ground,
        })
    } else {
        conn.write_packet(
            crate::protocol::packet::play::serverbound::PlayerPosition_HeadY {
                x: position.x,
                z: position.z,
                on_ground,
                feet_y: position.y,
                head_y: position.y + 1.62,
            },
        )
    }
}

pub fn send_look(conn: &mut Conn, yaw: f32, pitch: f32, on_ground: bool) -> Result<(), Error> {
    conn.write_packet(crate::protocol::packet::play::serverbound::PlayerLook {
        yaw: -yaw * (180.0 / PI),
        pitch: (-pitch * (180.0 / PI) + 180.0).min(90.0), // used to make sure, that we don't send impossible pitch values
        on_ground,
    })
}

pub fn send_flying(conn: &mut Conn, on_ground: bool) -> Result<(), Error> {
    conn.write_packet(crate::protocol::packet::play::serverbound::Player { on_ground })
}
