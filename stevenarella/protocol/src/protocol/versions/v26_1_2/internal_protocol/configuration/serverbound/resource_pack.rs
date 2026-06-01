use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt, UUID,
};

pub(crate) fn read_resource_pack_configuration_serverbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::serverbound::internal_ids::ConfigurationResourcePackServerbound => {
            let _id: UUID = Serializable::read_from(buf)?;
            let action: VarInt = Serializable::read_from(buf)?;
            Ok(Some(Packet::ResourcePackStatus(
                packet::play::serverbound::ResourcePackStatus { result: action },
            )))
        }
        _ => Ok(None),
    }
}
