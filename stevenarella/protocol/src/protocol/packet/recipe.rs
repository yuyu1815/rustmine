use std::io;

use crate::item;
use crate::protocol::{Error, LenPrefixed, Serializable, VarInt};

type RecipeIngredient = LenPrefixed<VarInt, Option<item::Stack>>;

#[derive(Debug, Default)]
pub enum RecipeData {
    Shapeless {
        group: String,
        ingredients: LenPrefixed<VarInt, RecipeIngredient>,
        result: Option<item::Stack>,
    },
    Shaped {
        width: VarInt,
        height: VarInt,
        group: String,
        ingredients: Vec<RecipeIngredient>,
        result: Option<item::Stack>,
    },
    #[default]
    ArmorDye,
    BookCloning,
    MapCloning,
    MapExtending,
    FireworkRocket,
    FireworkStar,
    FireworkStarFade,
    RepairItem,
    TippedArrow,
    BannerDuplicate,
    BannerAddPattern,
    ShieldDecoration,
    ShulkerBoxColoring,
    SuspiciousStew,
    Smelting {
        group: String,
        ingredient: RecipeIngredient,
        result: Option<item::Stack>,
        experience: f32,
        cooking_time: VarInt,
    },
    Blasting {
        group: String,
        ingredient: RecipeIngredient,
        result: Option<item::Stack>,
        experience: f32,
        cooking_time: VarInt,
    },
    Smoking {
        group: String,
        ingredient: RecipeIngredient,
        result: Option<item::Stack>,
        experience: f32,
        cooking_time: VarInt,
    },
    Campfire {
        group: String,
        ingredient: RecipeIngredient,
        result: Option<item::Stack>,
        experience: f32,
        cooking_time: VarInt,
    },
    Stonecutting {
        group: String,
        ingredient: RecipeIngredient,
        result: Option<item::Stack>,
    },
    Smithing {
        base: RecipeIngredient,
        addition: RecipeIngredient,
        result: Option<item::Stack>,
    },
}

#[derive(Debug, Default)]
pub struct Recipe {
    pub id: String,
    pub ty: String,
    pub data: RecipeData,
}

impl Serializable for Recipe {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        let (id, ty) = {
            let a = String::read_from(buf)?;
            let b = String::read_from(buf)?;

            let protocol_version = super::current_protocol_version();

            // 1.14+ swaps recipe identifier and type, and adds namespace to type
            if protocol_version >= 477 {
                let ty = a;
                let id = b;

                if ty.find(':').is_some() {
                    (id, ty)
                } else {
                    (id, format!("minecraft:{}", ty))
                }
            } else {
                let ty = b;
                let id = a;
                (id, format!("minecraft:{}", ty))
            }
        };

        let data = match ty.as_ref() {
            "minecraft:crafting_shapeless" => RecipeData::Shapeless {
                group: Serializable::read_from(buf)?,
                ingredients: Serializable::read_from(buf)?,
                result: Serializable::read_from(buf)?,
            },
            "minecraft:crafting_shaped" => {
                let width: VarInt = Serializable::read_from(buf)?;
                let height: VarInt = Serializable::read_from(buf)?;
                let group: String = Serializable::read_from(buf)?;

                let capacity = width.0 as usize * height.0 as usize;

                let mut ingredients = Vec::with_capacity(capacity);
                for _ in 0..capacity {
                    ingredients.push(Serializable::read_from(buf)?);
                }
                let result: Option<item::Stack> = Serializable::read_from(buf)?;

                RecipeData::Shaped {
                    width,
                    height,
                    group,
                    ingredients,
                    result,
                }
            }
            "minecraft:crafting_special_armordye" => RecipeData::ArmorDye,
            "minecraft:crafting_special_bookcloning" => RecipeData::BookCloning,
            "minecraft:crafting_special_mapcloning" => RecipeData::MapCloning,
            "minecraft:crafting_special_mapextending" => RecipeData::MapExtending,
            "minecraft:crafting_special_firework_rocket" => RecipeData::FireworkRocket,
            "minecraft:crafting_special_firework_star" => RecipeData::FireworkStar,
            "minecraft:crafting_special_firework_star_fade" => RecipeData::FireworkStarFade,
            "minecraft:crafting_special_repairitem" => RecipeData::RepairItem,
            "minecraft:crafting_special_tippedarrow" => RecipeData::TippedArrow,
            "minecraft:crafting_special_bannerduplicate" => RecipeData::BannerDuplicate,
            "minecraft:crafting_special_banneraddpattern" => RecipeData::BannerAddPattern,
            "minecraft:crafting_special_shielddecoration" => RecipeData::ShieldDecoration,
            "minecraft:crafting_special_shulkerboxcoloring" => RecipeData::ShulkerBoxColoring,
            "minecraft:crafting_special_suspiciousstew" => RecipeData::SuspiciousStew,
            "minecraft:smelting" => RecipeData::Smelting {
                group: Serializable::read_from(buf)?,
                ingredient: Serializable::read_from(buf)?,
                result: Serializable::read_from(buf)?,
                experience: Serializable::read_from(buf)?,
                cooking_time: Serializable::read_from(buf)?,
            },
            "minecraft:blasting" => RecipeData::Blasting {
                group: Serializable::read_from(buf)?,
                ingredient: Serializable::read_from(buf)?,
                result: Serializable::read_from(buf)?,
                experience: Serializable::read_from(buf)?,
                cooking_time: Serializable::read_from(buf)?,
            },
            "minecraft:smoking" => RecipeData::Smoking {
                group: Serializable::read_from(buf)?,
                ingredient: Serializable::read_from(buf)?,
                result: Serializable::read_from(buf)?,
                experience: Serializable::read_from(buf)?,
                cooking_time: Serializable::read_from(buf)?,
            },
            "minecraft:campfire" | "minecraft:campfire_cooking" => RecipeData::Campfire {
                group: Serializable::read_from(buf)?,
                ingredient: Serializable::read_from(buf)?,
                result: Serializable::read_from(buf)?,
                experience: Serializable::read_from(buf)?,
                cooking_time: Serializable::read_from(buf)?,
            },
            "minecraft:stonecutting" => RecipeData::Stonecutting {
                group: Serializable::read_from(buf)?,
                ingredient: Serializable::read_from(buf)?,
                result: Serializable::read_from(buf)?,
            },
            "minecraft:smithing" => RecipeData::Smithing {
                base: Serializable::read_from(buf)?,
                addition: Serializable::read_from(buf)?,
                result: Serializable::read_from(buf)?,
            },
            _ => panic!("unrecognized recipe type: {}", ty),
        };

        Ok(Recipe { id, ty, data })
    }

    fn write_to<W: io::Write>(&self, _: &mut W) -> Result<(), Error> {
        unimplemented!()
    }
}
