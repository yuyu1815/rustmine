use std::io;

use crate::protocol::{
    packet::{self, Packet},
    read_nbt_string_component, Error, Serializable,
};

pub(crate) fn read_text_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlaySetSubtitleTextClientbound => {
            Ok(Some(Packet::PlaySetSubtitleTextClientbound(
                packet::play::clientbound::PlaySetSubtitleTextClientbound {
                    text: read_nbt_string_component(buf)?,
                },
            )))
        }
        packet::play::clientbound::internal_ids::PlaySetTitleTextClientbound => {
            Ok(Some(Packet::PlaySetTitleTextClientbound(
                packet::play::clientbound::PlaySetTitleTextClientbound {
                    text: read_nbt_string_component(buf)?,
                },
            )))
        }
        packet::play::clientbound::internal_ids::PlaySystemChatClientbound => {
            let content = read_nbt_string_component(buf)?;
            let overlay = bool::read_from(buf)?;
            Ok(Some(Packet::PlaySystemChatClientbound(
                packet::play::clientbound::PlaySystemChatClientbound { content, overlay },
            )))
        }
        packet::play::clientbound::internal_ids::PlayTabListClientbound => Ok(Some(
            Packet::PlayTabListClientbound(packet::play::clientbound::PlayTabListClientbound {
                header: read_nbt_string_component(buf)?,
                footer: read_nbt_string_component(buf)?,
            }),
        )),
        _ => Ok(None),
    }
}
