use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(super) fn read_update_recipes_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
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
        _ => Ok(None),
    }
}
