use std::io;

use crate::nbt;
use crate::protocol::{Error, Serializable};

#[derive(Debug, Default)]
pub struct Trade {
    pub input_item_1: Option<nbt::NamedTag>,
    pub output_item: Option<nbt::NamedTag>,
    pub has_second_item: bool,
    pub input_item_2: Option<nbt::NamedTag>,
    pub trades_disabled: bool,
    pub tool_uses: i32,
    pub max_trade_uses: i32,
    pub xp: i32,
    pub special_price: i32,
    pub price_multiplier: f32,
    pub demand: Option<i32>,
}

impl Serializable for Trade {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        let protocol_version = super::super::current_protocol_version();

        Ok(Trade {
            input_item_1: Serializable::read_from(buf)?,
            output_item: Serializable::read_from(buf)?,
            has_second_item: Serializable::read_from(buf)?,
            input_item_2: Serializable::read_from(buf)?,
            trades_disabled: Serializable::read_from(buf)?,
            tool_uses: Serializable::read_from(buf)?,
            max_trade_uses: Serializable::read_from(buf)?,
            xp: Serializable::read_from(buf)?,
            special_price: Serializable::read_from(buf)?,
            price_multiplier: Serializable::read_from(buf)?,
            demand: if protocol_version >= 498 {
                Some(Serializable::read_from(buf)?)
            } else {
                None
            },
        })
    }

    fn write_to<W: io::Write>(&self, _: &mut W) -> Result<(), Error> {
        unimplemented!()
    }
}
