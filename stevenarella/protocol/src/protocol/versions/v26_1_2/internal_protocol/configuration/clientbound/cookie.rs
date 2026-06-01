use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(crate) fn read_cookie_configuration_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::clientbound::internal_ids::ConfigurationCookieRequestClientbound => {
            let _packet =
                packet::configuration::clientbound::ConfigurationCookieRequestClientbound {
                    key: Serializable::read_from(buf)?,
                };
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "CookieRequest".to_owned(),
                    data: Vec::new(),
                },
            )))
        }
        packet::configuration::clientbound::internal_ids::ConfigurationStoreCookieClientbound => {
            let key: String = Serializable::read_from(buf)?;
            let payload_len = VarInt::read_from(buf)?.0;
            if payload_len < 0 {
                return Err(Error::Err(format!(
                    "negative store_cookie payload length {}",
                    payload_len
                )));
            }
            let mut payload = vec![0; payload_len as usize];
            buf.read_exact(&mut payload)?;
            let packet = packet::configuration::clientbound::ConfigurationStoreCookieClientbound {
                key,
                payload,
            };
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "StoreCookie".to_owned(),
                    data: packet.payload,
                },
            )))
        }
        _ => Ok(None),
    }
}
