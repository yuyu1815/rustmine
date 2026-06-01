use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

mod sound_entity;

pub(crate) fn read_sound_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    if let Some(packet) =
        sound_entity::read_sound_entity_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }

    match internal_id {
        packet::play::clientbound::internal_ids::PlaySoundClientbound => {
            let sound_holder_id = VarInt::read_from(buf)?;
            if sound_holder_id.0 != 8 {
                return Err(Error::Err(format!(
                    "unsupported Play sound SoundEvent holder id {}",
                    sound_holder_id.0
                )));
            }
            let source = VarInt::read_from(buf)?;
            if source.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported Play sound source id {}",
                    source.0
                )));
            }
            Ok(Some(Packet::PlaySoundClientbound(
                packet::play::clientbound::PlaySoundClientbound {
                    sound_holder_id,
                    source,
                    x: i32::read_from(buf)?,
                    y: i32::read_from(buf)?,
                    z: i32::read_from(buf)?,
                    volume: f32::read_from(buf)?,
                    pitch: f32::read_from(buf)?,
                    seed: i64::read_from(buf)?,
                },
            )))
        }
        _ => Ok(None),
    }
}
