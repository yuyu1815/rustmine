use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(crate) fn read_update_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    if let Some(packet) =
        super::update_attributes::read_update_attributes_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        super::update_tags::read_update_tags_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        super::update_recipes::read_update_recipes_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }

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
