use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable,
};

pub(crate) fn read_finish_configuration_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::clientbound::internal_ids::ConfigurationFinishConfigurationClientbound => {
            let _: () = Serializable::read_from(buf)?;
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "FinishConfiguration".to_owned(),
                    data: Vec::new(),
                },
            )))
        }
        _ => Ok(None),
    }
}

pub(crate) fn read_finish_configuration_serverbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::serverbound::internal_ids::ConfigurationFinishConfigurationServerbound => {
            let _: () = Serializable::read_from(buf)?;
            Ok(Some(Packet::PluginMessageServerbound(
                packet::play::serverbound::PluginMessageServerbound {
                    channel: "FinishConfiguration".to_owned(),
                    data: Vec::new(),
                },
            )))
        }
        _ => Ok(None),
    }
}
