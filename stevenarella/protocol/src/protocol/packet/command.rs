use std::io;

use crate::protocol::{Error, LenPrefixed, Serializable, VarInt};

#[derive(Debug, Default)]
pub struct CommandNode {
    pub flags: u8,
    pub children: LenPrefixed<VarInt, VarInt>,
    pub redirect_node: Option<VarInt>,
    pub name: Option<String>,
    pub parser: Option<String>,
    pub properties: Option<CommandProperty>,
    pub suggestions_type: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
enum CommandNodeType {
    Root,
    Literal,
    Argument,
}

#[derive(Debug)]
pub enum CommandProperty {
    Bool,
    Double {
        flags: u8,
        min: Option<f64>,
        max: Option<f64>,
    },
    Float {
        flags: u8,
        min: Option<f32>,
        max: Option<f32>,
    },
    Integer {
        flags: u8,
        min: Option<i32>,
        max: Option<i32>,
    },
    String {
        token_type: VarInt,
    },
    Entity {
        flags: u8,
    },
    Angle,
    GameProfile,
    BlockPos,
    ColumnPos,
    Time,
    Vec3,
    Vec2,
    BlockState,
    BlockPredicate,
    ItemStack,
    ItemPredicate,
    Color,
    Component,
    Message,
    Nbt,
    NbtPath,
    NbtTag,
    NbtCompoundTag,
    Objective,
    ObjectiveCriteria,
    Operation,
    Particle,
    Rotation,
    ScoreboardSlot,
    ScoreHolder {
        flags: u8,
    },
    Swizzle,
    Team,
    ItemSlot,
    ResourceLocation,
    MobEffect,
    Function,
    EntityAnchor,
    Range {
        decimals: bool,
    },
    IntRange,
    FloatRange,
    ItemEnchantment,
    EntitySummon,
    Dimension,
    UUID,
    ForgeModId,
    ForgeEnum {
        cls: String,
    },
}

impl Serializable for CommandNode {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        let flags: u8 = Serializable::read_from(buf)?;
        let children: LenPrefixed<VarInt, VarInt> = Serializable::read_from(buf)?;

        let node_type = match flags & 0x03 {
            0 => CommandNodeType::Root,
            1 => CommandNodeType::Literal,
            2 => CommandNodeType::Argument,
            _ => panic!("unrecognized command node type {}", flags & 0x03),
        };
        let _is_executable = flags & 0x04 != 0;
        let has_redirect = flags & 0x08 != 0;
        let has_suggestions_type = flags & 0x10 != 0;

        let redirect_node: Option<VarInt> = if has_redirect {
            Some(Serializable::read_from(buf)?)
        } else {
            None
        };

        let name: Option<String> =
            if node_type == CommandNodeType::Argument || node_type == CommandNodeType::Literal {
                Serializable::read_from(buf)?
            } else {
                None
            };
        let parser: Option<String> = if node_type == CommandNodeType::Argument {
            Serializable::read_from(buf)?
        } else {
            None
        };

        let properties: Option<CommandProperty> = if let Some(ref parse) = parser {
            Some(match parse.as_ref() {
                "brigadier:bool" => CommandProperty::Bool,
                "brigadier:double" => {
                    let flags = Serializable::read_from(buf)?;
                    let min = if flags & 0x01 != 0 {
                        Some(Serializable::read_from(buf)?)
                    } else {
                        None
                    };
                    let max = if flags & 0x02 != 0 {
                        Some(Serializable::read_from(buf)?)
                    } else {
                        None
                    };
                    CommandProperty::Double { flags, min, max }
                }
                "brigadier:float" => {
                    let flags = Serializable::read_from(buf)?;
                    let min = if flags & 0x01 != 0 {
                        Some(Serializable::read_from(buf)?)
                    } else {
                        None
                    };
                    let max = if flags & 0x02 != 0 {
                        Some(Serializable::read_from(buf)?)
                    } else {
                        None
                    };
                    CommandProperty::Float { flags, min, max }
                }
                "brigadier:integer" => {
                    let flags = Serializable::read_from(buf)?;
                    let min = if flags & 0x01 != 0 {
                        Some(Serializable::read_from(buf)?)
                    } else {
                        None
                    };
                    let max = if flags & 0x02 != 0 {
                        Some(Serializable::read_from(buf)?)
                    } else {
                        None
                    };
                    CommandProperty::Integer { flags, min, max }
                }
                "brigadier:string" => CommandProperty::String {
                    token_type: Serializable::read_from(buf)?,
                },
                "minecraft:entity" => CommandProperty::Entity {
                    flags: Serializable::read_from(buf)?,
                },
                "minecraft:angle" => CommandProperty::Angle,
                "minecraft:game_profile" => CommandProperty::GameProfile,
                "minecraft:block_pos" => CommandProperty::BlockPos,
                "minecraft:column_pos" => CommandProperty::ColumnPos,
                "minecraft:time" => CommandProperty::Time,
                "minecraft:vec3" => CommandProperty::Vec3,
                "minecraft:vec2" => CommandProperty::Vec2,
                "minecraft:block_state" => CommandProperty::BlockState,
                "minecraft:block_predicate" => CommandProperty::BlockPredicate,
                "minecraft:item_stack" => CommandProperty::ItemStack,
                "minecraft:item_predicate" => CommandProperty::ItemPredicate,
                "minecraft:color" => CommandProperty::Color,
                "minecraft:component" => CommandProperty::Component,
                "minecraft:message" => CommandProperty::Message,
                "minecraft:nbt" => CommandProperty::Nbt,
                "minecraft:nbt_path" => CommandProperty::NbtPath,
                "minecraft:nbt_tag" => CommandProperty::NbtTag,
                "minecraft:nbt_compound_tag" => CommandProperty::NbtCompoundTag,
                "minecraft:objective" => CommandProperty::Objective,
                "minecraft:objective_criteria" => CommandProperty::ObjectiveCriteria,
                "minecraft:operation" => CommandProperty::Operation,
                "minecraft:particle" => CommandProperty::Particle,
                "minecraft:rotation" => CommandProperty::Rotation,
                "minecraft:scoreboard_slot" => CommandProperty::ScoreboardSlot,
                "minecraft:score_holder" => CommandProperty::ScoreHolder {
                    flags: Serializable::read_from(buf)?,
                },
                "minecraft:swizzle" => CommandProperty::Swizzle,
                "minecraft:team" => CommandProperty::Team,
                "minecraft:item_slot" => CommandProperty::ItemSlot,
                "minecraft:resource_location" => CommandProperty::ResourceLocation,
                "minecraft:mob_effect" => CommandProperty::MobEffect,
                "minecraft:function" => CommandProperty::Function,
                "minecraft:entity_anchor" => CommandProperty::EntityAnchor,
                "minecraft:range" => CommandProperty::Range {
                    decimals: Serializable::read_from(buf)?,
                },
                "minecraft:int_range" => CommandProperty::IntRange,
                "minecraft:float_range" => CommandProperty::FloatRange,
                "minecraft:item_enchantment" => CommandProperty::ItemEnchantment,
                "minecraft:entity_summon" => CommandProperty::EntitySummon,
                "minecraft:dimension" => CommandProperty::Dimension,
                "minecraft:uuid" => CommandProperty::UUID,
                "forge:modid" => CommandProperty::ForgeModId,
                "forge:enum" => CommandProperty::ForgeEnum {
                    cls: Serializable::read_from(buf)?,
                },
                _ => panic!("unsupported command node parser {}", parse),
            })
        } else {
            None
        };

        let suggestions_type: Option<String> = if has_suggestions_type {
            Serializable::read_from(buf)?
        } else {
            None
        };

        Ok(CommandNode {
            flags,
            children,
            redirect_node,
            name,
            parser,
            properties,
            suggestions_type,
        })
    }

    fn write_to<W: io::Write>(&self, _: &mut W) -> Result<(), Error> {
        unimplemented!()
    }
}
