use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(crate) fn read_update_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlayUpdateAttributesClientbound => {
            let entity_id = VarInt::read_from(buf)?;
            let attribute_count = VarInt::read_from(buf)?;
            if attribute_count.0 < 0 {
                return Err(Error::Err(format!(
                    "negative Play update_attributes attribute count {}",
                    attribute_count.0
                )));
            }
            if attribute_count.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play update_attributes attribute count {}",
                    attribute_count.0
                )));
            }
            Ok(Some(Packet::PlayUpdateAttributesClientbound(
                packet::play::clientbound::PlayUpdateAttributesClientbound {
                    entity_id,
                    attribute_count,
                },
            )))
        }
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
        packet::play::clientbound::internal_ids::PlayUpdateRecipesClientbound => {
            let item_set_count = VarInt::read_from(buf)?;
            let stonecutter_recipe_count = VarInt::read_from(buf)?;
            for (name, count) in [
                ("item set", item_set_count.0),
                ("stonecutter recipe", stonecutter_recipe_count.0),
            ] {
                if count < 0 {
                    return Err(Error::Err(format!(
                        "negative Play update_recipes {} count {}",
                        name, count
                    )));
                }
                if count != 0 {
                    return Err(Error::Err(format!(
                        "unsupported non-empty Play update_recipes {} count {}",
                        name, count
                    )));
                }
            }
            Ok(Some(Packet::PlayUpdateRecipesClientbound(
                packet::play::clientbound::PlayUpdateRecipesClientbound {
                    item_set_count,
                    stonecutter_recipe_count,
                },
            )))
        }
        packet::play::clientbound::internal_ids::PlayUpdateTagsClientbound => {
            let registry_payload_count = VarInt::read_from(buf)?;
            if registry_payload_count.0 < 0 {
                return Err(Error::Err(format!(
                    "negative Play update_tags registry payload count {}",
                    registry_payload_count.0
                )));
            }
            if registry_payload_count.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play update_tags registry payload count {}",
                    registry_payload_count.0
                )));
            }
            Ok(Some(Packet::PlayUpdateTagsClientbound(
                packet::play::clientbound::PlayUpdateTagsClientbound {
                    registry_payload_count,
                },
            )))
        }
        _ => Ok(None),
    }
}
