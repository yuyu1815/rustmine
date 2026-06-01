use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable,
};

pub(crate) fn read_keep_alive_configuration_serverbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::serverbound::internal_ids::ConfigurationKeepAliveServerbound_i64 => {
            let packet = packet::play::serverbound::KeepAliveServerbound_i64 {
                id: Serializable::read_from(buf)?,
            };
            Ok(Some(Packet::KeepAliveServerbound_i64(packet)))
        }
        _ => Ok(None),
    }
}
