use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable,
};

pub(crate) fn read_dialog_configuration_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::clientbound::internal_ids::ConfigurationClearDialogClientbound => {
            let _packet = packet::configuration::clientbound::ConfigurationClearDialogClientbound {
                empty: Serializable::read_from(buf)?,
            };
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "ClearDialog".to_owned(),
                    data: Vec::new(),
                },
            )))
        }
        packet::configuration::clientbound::internal_ids::ConfigurationShowDialogClientbound => {
            let mut dialog_data = Vec::new();
            buf.read_to_end(&mut dialog_data)?;
            let packet = packet::configuration::clientbound::ConfigurationShowDialogClientbound {
                dialog_data,
            };
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "ShowDialog".to_owned(),
                    data: packet.dialog_data,
                },
            )))
        }
        _ => Ok(None),
    }
}
