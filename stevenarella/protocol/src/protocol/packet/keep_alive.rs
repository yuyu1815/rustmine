use crate::protocol::{packet, Conn, Error, State, VarInt};
use crate::shared::Version;

pub fn send_keep_alive(conn: &mut Conn, id: i64) -> Result<(), Error> {
    if conn.state == State::Configuration {
        return conn.write_packet(
            packet::configuration::serverbound::ConfigurationKeepAliveServerbound_i64 { id },
        );
    }

    let version = conn.get_version();
    if version < Version::V1_8 {
        conn.write_packet(packet::play::serverbound::KeepAliveServerbound_i32 { id: id as i32 })
    } else if version < Version::V1_12 {
        conn.write_packet(packet::play::serverbound::KeepAliveServerbound_VarInt {
            id: VarInt(id as i32),
        })
    } else {
        conn.write_packet(packet::play::serverbound::KeepAliveServerbound_i64 { id })
    }
}
