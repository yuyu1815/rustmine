use std::io;

use crate::protocol::{
    packet::{self, Packet},
    read_nbt_string_component, Error, Serializable,
};

pub(crate) fn read_disconnect_reset_chat_configuration_clientbound_packet_by_internal_id<
    R: io::Read,
>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::clientbound::internal_ids::ConfigurationDisconnectClientbound => {
            let packet = packet::configuration::clientbound::ConfigurationDisconnectClientbound {
                reason: read_nbt_string_component(buf)?,
            };
            Ok(Some(Packet::Disconnect(
                packet::play::clientbound::Disconnect {
                    reason: packet.reason,
                },
            )))
        }
        packet::configuration::clientbound::internal_ids::ConfigurationResetChatClientbound => {
            let _: () = Serializable::read_from(buf)?;
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "ResetChat".to_owned(),
                    data: Vec::new(),
                },
            )))
        }
        _ => Ok(None),
    }
}
