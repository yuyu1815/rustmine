use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable,
};

pub(crate) fn read_pong_configuration_serverbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::serverbound::internal_ids::ConfigurationPongServerbound_i32 => {
            let id: i32 = Serializable::read_from(buf)?;
            Ok(Some(Packet::StatusPong(
                packet::status::clientbound::StatusPong { ping: id.into() },
            )))
        }
        _ => Ok(None),
    }
}
