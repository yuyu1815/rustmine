use std::io;

use crate::protocol::{
    packet::{self, Packet},
    read_nbt_string_component, Error, Serializable,
};

pub(super) fn read_system_chat_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlaySystemChatClientbound => {
            let content = read_nbt_string_component(buf)?;
            let overlay = bool::read_from(buf)?;
            Ok(Some(Packet::PlaySystemChatClientbound(
                packet::play::clientbound::PlaySystemChatClientbound { content, overlay },
            )))
        }
        _ => Ok(None),
    }
}
