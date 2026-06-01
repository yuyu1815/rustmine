use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable,
};

pub(crate) fn read_dialog_play_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlayClearDialogClientbound => {
            Ok(Some(Packet::PlayClearDialogClientbound(
                packet::play::clientbound::PlayClearDialogClientbound {
                    empty: Serializable::read_from(buf)?,
                },
            )))
        }
        packet::play::clientbound::internal_ids::PlayShowDialogClientbound => {
            let mut dialog_data = Vec::new();
            buf.read_to_end(&mut dialog_data)?;
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "ShowDialog".to_owned(),
                    data: dialog_data,
                },
            )))
        }
        _ => Ok(None),
    }
}
