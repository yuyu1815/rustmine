use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable,
};

pub(crate) fn read_keep_alive_ping_configuration_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::clientbound::internal_ids::ConfigurationKeepAliveClientbound_i64 => {
            let packet = packet::play::clientbound::KeepAliveClientbound_i64 {
                id: Serializable::read_from(buf)?,
            };
            Ok(Some(Packet::KeepAliveClientbound_i64(packet)))
        }
        packet::configuration::clientbound::internal_ids::ConfigurationPingClientbound_i32 => {
            let id: i32 = Serializable::read_from(buf)?;
            Ok(Some(Packet::StatusPing(
                packet::status::serverbound::StatusPing { ping: id.into() },
            )))
        }
        _ => Ok(None),
    }
}
