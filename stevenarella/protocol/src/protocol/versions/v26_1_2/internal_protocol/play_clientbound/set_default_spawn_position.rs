use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable,
};
use crate::shared::Position;

pub(crate) fn read_set_default_spawn_position_play_clientbound_packet_by_internal_id<
    R: io::Read,
>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlaySetDefaultSpawnPositionClientbound => {
            let dimension = String::read_from(buf)?;
            if dimension != "minecraft:overworld" {
                return Err(Error::Err(format!(
                    "unsupported Play set_default_spawn_position dimension {:?}",
                    dimension
                )));
            }
            Ok(Some(Packet::PlaySetDefaultSpawnPositionClientbound(
                packet::play::clientbound::PlaySetDefaultSpawnPositionClientbound {
                    dimension,
                    location: Position::read_from(buf)?,
                    yaw: buf.read_f32::<BigEndian>()?,
                    pitch: buf.read_f32::<BigEndian>()?,
                },
            )))
        }
        _ => Ok(None),
    }
}
