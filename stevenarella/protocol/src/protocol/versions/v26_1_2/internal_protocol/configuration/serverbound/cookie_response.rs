use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable,
};

pub(crate) fn read_cookie_response_configuration_serverbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::serverbound::internal_ids::ConfigurationCookieResponseServerbound => {
            let packet =
                packet::configuration::serverbound::ConfigurationCookieResponseServerbound {
                    key: Serializable::read_from(buf)?,
                    payload: if bool::read_from(buf)? {
                        Some(Serializable::read_from(buf)?)
                    } else {
                        None
                    },
                };
            Ok(Some(Packet::PluginMessageServerbound(
                packet::play::serverbound::PluginMessageServerbound {
                    channel: "CookieResponse".to_owned(),
                    data: packet
                        .payload
                        .map(|payload| payload.data)
                        .unwrap_or_else(Vec::new),
                },
            )))
        }
        _ => Ok(None),
    }
}
