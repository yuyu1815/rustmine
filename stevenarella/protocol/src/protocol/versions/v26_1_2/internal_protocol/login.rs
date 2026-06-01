use std::io;

use crate::protocol::{
    packet, read_lenient_json_component, Direction, Error, Serializable, State, VarInt, UUID,
};

use super::super::translate_internal_packet_id;

mod serverbound;

pub(crate) fn read_login_packet_by_id<R: io::Read>(
    dir: Direction,
    id: i32,
    buf: &mut R,
) -> Result<Option<packet::Packet>, Error> {
    match dir {
        Direction::Serverbound => {
            let internal_id =
                translate_internal_packet_id(State::Login, Direction::Serverbound, id, true);
            serverbound::read_login_serverbound_packet_by_internal_id(internal_id, buf)
        }
        Direction::Clientbound => read_login_clientbound_packet(id, buf),
    }
}

fn read_login_clientbound_packet<R: io::Read>(
    id: i32,
    buf: &mut R,
) -> Result<Option<packet::Packet>, Error> {
    let internal_id = translate_internal_packet_id(State::Login, Direction::Clientbound, id, true);
    if internal_id == packet::login::clientbound::internal_ids::LoginDisconnect {
        return Ok(Some(packet::Packet::LoginDisconnect(
            packet::login::clientbound::LoginDisconnect {
                reason: read_lenient_json_component(buf)?,
            },
        )));
    }
    if internal_id == packet::login::clientbound::internal_ids::LoginSuccess_UUID {
        let uuid: UUID = Serializable::read_from(buf)?;
        let username: String = Serializable::read_from(buf)?;
        let property_count: VarInt = Serializable::read_from(buf)?;
        for _ in 0..property_count.0 {
            let _property_name: String = Serializable::read_from(buf)?;
            let _property_value: String = Serializable::read_from(buf)?;
            let has_signature: bool = Serializable::read_from(buf)?;
            if has_signature {
                let _property_signature: String = Serializable::read_from(buf)?;
            }
        }
        return Ok(Some(packet::Packet::LoginSuccess_UUID(
            packet::login::clientbound::LoginSuccess_UUID { uuid, username },
        )));
    }
    Ok(None)
}
