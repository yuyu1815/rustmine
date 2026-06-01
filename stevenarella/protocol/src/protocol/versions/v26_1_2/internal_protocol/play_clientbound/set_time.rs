use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(crate) fn read_set_time_play_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlaySetTimeClientbound => {
            let game_time = i64::read_from(buf)?;
            let clock_update_count = VarInt::read_from(buf)?;
            if clock_update_count.0 < 0 {
                return Err(Error::Err(format!(
                    "negative Play set_time clock update count {}",
                    clock_update_count.0
                )));
            }
            if clock_update_count.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play set_time clock update count {}",
                    clock_update_count.0
                )));
            }
            Ok(Some(Packet::PlaySetTimeClientbound(
                packet::play::clientbound::PlaySetTimeClientbound {
                    game_time,
                    clock_update_count,
                },
            )))
        }
        _ => Ok(None),
    }
}
