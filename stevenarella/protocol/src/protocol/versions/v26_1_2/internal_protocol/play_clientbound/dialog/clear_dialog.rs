use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable,
};

pub(super) fn read_clear_dialog_play_clientbound_packet_by_internal_id<R: io::Read>(
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
        _ => Ok(None),
    }
}
