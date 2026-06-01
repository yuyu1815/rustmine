use std::io;

use crate::protocol::{packet, Error, Serializable, UUID};

pub(super) fn read_login_serverbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<packet::Packet>, Error> {
    if internal_id == packet::login::serverbound::internal_ids::LoginStart {
        let username: String = Serializable::read_from(buf)?;
        let _profile_id: UUID = Serializable::read_from(buf)?;
        return Ok(Some(packet::Packet::LoginStart(
            packet::login::serverbound::LoginStart { username },
        )));
    }
    Ok(None)
}
