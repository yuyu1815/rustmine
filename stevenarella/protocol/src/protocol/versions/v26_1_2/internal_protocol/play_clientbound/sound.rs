use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(crate) fn read_sound_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlaySoundEntityClientbound => {
            let sound_holder_id = VarInt::read_from(buf)?;
            if sound_holder_id.0 != 8 {
                return Err(Error::Err(format!(
                    "unsupported Play sound_entity SoundEvent holder id {}",
                    sound_holder_id.0
                )));
            }
            let source = VarInt::read_from(buf)?;
            if source.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported Play sound_entity source id {}",
                    source.0
                )));
            }
            let entity_id = VarInt::read_from(buf)?;
            if entity_id.0 != 1 {
                return Err(Error::Err(format!(
                    "unsupported Play sound_entity fixture entity id {}",
                    entity_id.0
                )));
            }
            let volume = f32::read_from(buf)?;
            if volume != 0.75 {
                return Err(Error::Err(format!(
                    "unsupported Play sound_entity volume {}",
                    volume
                )));
            }
            let pitch = f32::read_from(buf)?;
            if pitch != 1.25 {
                return Err(Error::Err(format!(
                    "unsupported Play sound_entity pitch {}",
                    pitch
                )));
            }
            let seed = i64::read_from(buf)?;
            if seed != 123456789 {
                return Err(Error::Err(format!(
                    "unsupported Play sound_entity seed {}",
                    seed
                )));
            }
            Ok(Some(Packet::PlaySoundEntityClientbound(
                packet::play::clientbound::PlaySoundEntityClientbound {
                    sound_holder_id,
                    source,
                    entity_id,
                    volume,
                    pitch,
                    seed,
                },
            )))
        }
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
