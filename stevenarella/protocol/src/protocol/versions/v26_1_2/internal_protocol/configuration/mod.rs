use std::io;

use crate::protocol::{packet::Packet, Direction, Error};

mod clientbound;
mod finish_configuration;
mod serverbound;

pub(crate) fn read_configuration_packet_by_id<R: io::Read>(
    dir: Direction,
    id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match dir {
        Direction::Serverbound => serverbound::read_configuration_serverbound_packet_by_id(id, buf),
        Direction::Clientbound => clientbound::read_configuration_clientbound_packet_by_id(id, buf),
    }
}
