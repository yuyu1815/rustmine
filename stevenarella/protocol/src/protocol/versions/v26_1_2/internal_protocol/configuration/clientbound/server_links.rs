use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(crate) fn read_server_links_configuration_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::clientbound::internal_ids::ConfigurationServerLinksClientbound => {
            let link_count = VarInt::read_from(buf)?.0;
            if link_count < 0 {
                return Err(Error::Err(format!(
                    "negative server_links link count {}",
                    link_count
                )));
            }
            if link_count != 0 {
                return Err(Error::Err(format!(
                    "unsupported non-empty server_links list count {}",
                    link_count
                )));
            }
            let _packet = packet::configuration::clientbound::ConfigurationServerLinksClientbound {
                link_count,
                links_data: Vec::new(),
            };
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "ServerLinks".to_owned(),
                    data: Vec::new(),
                },
            )))
        }
        _ => Ok(None),
    }
}
