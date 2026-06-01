use std::io;

use crate::item;
use crate::protocol::{Error, LenPrefixed, Serializable, VarInt, UUID};

#[derive(Debug, Default)]
pub struct EntityEquipment {
    pub slot: u8,
    pub item: Option<item::Stack>,
}

impl Serializable for EntityEquipment {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        Ok(EntityEquipment {
            slot: Serializable::read_from(buf)?,
            item: Serializable::read_from(buf)?,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.slot.write_to(buf)?;
        self.item.write_to(buf)
    }
}

// Top-bit terminated array of EntityEquipment
#[derive(Debug, Default)]
pub struct EntityEquipments {
    pub equipments: Vec<EntityEquipment>,
}

impl Serializable for EntityEquipments {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        let mut equipments: Vec<EntityEquipment> = vec![];

        loop {
            let e: EntityEquipment = Serializable::read_from(buf)?;
            equipments.push(EntityEquipment {
                slot: e.slot & 0x7f,
                item: e.item,
            });

            if e.slot & 0x80 == 0 {
                break;
            }
            // TODO: detect infinite loop
        }

        Ok(EntityEquipments { equipments })
    }

    fn write_to<W: io::Write>(&self, _buf: &mut W) -> Result<(), Error> {
        unimplemented!()
    }
}

#[derive(Debug, Default)]
pub struct EntityProperty {
    pub key: String,
    pub value: f64,
    pub modifiers: LenPrefixed<VarInt, PropertyModifier>,
}

impl Serializable for EntityProperty {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        Ok(EntityProperty {
            key: Serializable::read_from(buf)?,
            value: Serializable::read_from(buf)?,
            modifiers: Serializable::read_from(buf)?,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.key.write_to(buf)?;
        self.value.write_to(buf)?;
        self.modifiers.write_to(buf)
    }
}

#[derive(Debug, Default)]
pub struct EntityProperty_i16 {
    pub key: String,
    pub value: f64,
    pub modifiers: LenPrefixed<i16, PropertyModifier>,
}

impl Serializable for EntityProperty_i16 {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        Ok(EntityProperty_i16 {
            key: Serializable::read_from(buf)?,
            value: Serializable::read_from(buf)?,
            modifiers: Serializable::read_from(buf)?,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.key.write_to(buf)?;
        self.value.write_to(buf)?;
        self.modifiers.write_to(buf)
    }
}

#[derive(Debug, Default)]
pub struct PropertyModifier {
    pub uuid: UUID,
    pub amount: f64,
    pub operation: i8,
}

impl Serializable for PropertyModifier {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        Ok(PropertyModifier {
            uuid: Serializable::read_from(buf)?,
            amount: Serializable::read_from(buf)?,
            operation: Serializable::read_from(buf)?,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.uuid.write_to(buf)?;
        self.amount.write_to(buf)?;
        self.operation.write_to(buf)
    }
}
