use std::io;

use crate::protocol::{packet, Direction, Error, State};

use super::super::translate_internal_packet_id;

mod clientbound;
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
        Direction::Clientbound => {
            let internal_id =
                translate_internal_packet_id(State::Login, Direction::Clientbound, id, true);
            clientbound::read_login_clientbound_packet_by_internal_id(internal_id, buf)
        }
    }
}
