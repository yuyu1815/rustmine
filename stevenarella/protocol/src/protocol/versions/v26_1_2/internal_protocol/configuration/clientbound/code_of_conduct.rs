use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable,
};

pub(crate) fn read_code_of_conduct_configuration_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::clientbound::internal_ids::ConfigurationCodeOfConductClientbound => {
            let packet =
                packet::configuration::clientbound::ConfigurationCodeOfConductClientbound {
                    code_of_conduct: Serializable::read_from(buf)?,
                };
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "CodeOfConduct".to_owned(),
                    data: packet.code_of_conduct.into_bytes(),
                },
            )))
        }
        _ => Ok(None),
    }
}
