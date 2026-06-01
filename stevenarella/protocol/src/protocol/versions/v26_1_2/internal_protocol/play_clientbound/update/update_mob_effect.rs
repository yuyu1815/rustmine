use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(super) fn read_update_mob_effect_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlayUpdateMobEffectClientbound => {
            let entity_id = VarInt::read_from(buf)?;
            let effect_holder_id = VarInt::read_from(buf)?;
            if effect_holder_id.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported Play update_mob_effect MobEffect holder id {}",
                    effect_holder_id.0
                )));
            }
            Ok(Some(Packet::PlayUpdateMobEffectClientbound(
                packet::play::clientbound::PlayUpdateMobEffectClientbound {
                    entity_id,
                    effect_holder_id,
                    amplifier: VarInt::read_from(buf)?,
                    duration: VarInt::read_from(buf)?,
                    flags: u8::read_from(buf)?,
                },
            )))
        }
        _ => Ok(None),
    }
}
