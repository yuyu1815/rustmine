use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(super) fn read_update_advancements_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlayUpdateAdvancementsClientbound => {
            let reset = bool::read_from(buf)?;
            let added_count = VarInt::read_from(buf)?;
            let removed_count = VarInt::read_from(buf)?;
            let progress_count = VarInt::read_from(buf)?;
            for (name, count) in [
                ("added", added_count.0),
                ("removed", removed_count.0),
                ("progress", progress_count.0),
            ] {
                if count < 0 {
                    return Err(Error::Err(format!(
                        "negative Play update_advancements {} count {}",
                        name, count
                    )));
                }
                if count != 0 {
                    return Err(Error::Err(format!(
                        "unsupported non-empty Play update_advancements {} count {}",
                        name, count
                    )));
                }
            }
            Ok(Some(Packet::PlayUpdateAdvancementsClientbound(
                packet::play::clientbound::PlayUpdateAdvancementsClientbound {
                    reset,
                    added_count,
                    removed_count,
                    progress_count,
                    show_advancements: bool::read_from(buf)?,
                },
            )))
        }
        _ => Ok(None),
    }
}
