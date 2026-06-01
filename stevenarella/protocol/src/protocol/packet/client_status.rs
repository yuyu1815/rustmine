use crate::protocol::{Conn, Error, VarInt};
use crate::shared::Version;

#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum ClientStatus {
    PerformRespawn = 0,
    RequestStats = 1,
    // this variant isn't available on all versions
    OpenInventory = 2,
}

pub fn send_client_status(conn: &mut Conn, status: ClientStatus) -> Result<(), Error> {
    let version = conn.get_version();
    // we don't send any information to the server when opening the inv in newer versions
    if version > Version::V1_11 && status == ClientStatus::OpenInventory {
        return Ok(());
    }

    if version < Version::V1_8 {
        conn.write_packet(
            crate::protocol::packet::play::serverbound::ClientStatus_u8 {
                action_id: status as u8,
            },
        )
    } else {
        conn.write_packet(crate::protocol::packet::play::serverbound::ClientStatus {
            action_id: VarInt(status as u8 as i32),
        })
    }
}
