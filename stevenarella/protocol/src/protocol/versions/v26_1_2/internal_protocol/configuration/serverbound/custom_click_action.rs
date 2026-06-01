use std::io;

use crate::protocol::{
    nbt,
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(crate) fn read_custom_click_action_configuration_serverbound_packet_by_internal_id<
    R: io::Read,
>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::serverbound::internal_ids::ConfigurationCustomClickActionServerbound => {
            let id: String = Serializable::read_from(buf)?;
            let payload_len: VarInt = Serializable::read_from(buf)?;
            let payload = if payload_len.0 > 0 {
                let mut payload_bytes = vec![0; payload_len.0 as usize];
                io::Read::read_exact(buf, &mut payload_bytes)?;
                let mut payload_slice = payload_bytes.as_slice();
                let payload_type: u8 = Serializable::read_from(&mut payload_slice)?;
                if payload_type != 10 {
                    return Err(Error::Err(
                        "custom_click_action payload is not a compound tag".to_owned(),
                    ));
                }
                Some(nbt::NamedTag(
                    String::new(),
                    nbt::Tag::read_from(&mut payload_slice)?,
                ))
            } else {
                None
            };
            let _packet = packet::configuration::serverbound::ConfigurationCustomClickActionServerbound {
                id,
                payload,
            };
            Ok(Some(Packet::PluginMessageServerbound(
                packet::play::serverbound::PluginMessageServerbound {
                    channel: "CustomClickAction".to_owned(),
                    data: Vec::new(),
                },
            )))
        }
        _ => Ok(None),
    }
}
