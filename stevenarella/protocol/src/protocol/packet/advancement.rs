use std::io;

use crate::item;
use crate::protocol::{Error, LenPrefixed, Serializable, VarInt};

#[derive(Debug, Default)]
pub struct Advancement {
    pub id: String,
    pub parent_id: Option<String>,
    pub display_data: Option<AdvancementDisplay>,
    pub criteria: LenPrefixed<VarInt, String>,
    pub requirements: LenPrefixed<VarInt, LenPrefixed<VarInt, String>>,
}

impl Serializable for Advancement {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        let id: String = Serializable::read_from(buf)?;
        let parent_id = {
            let has_parent: u8 = Serializable::read_from(buf)?;
            if has_parent != 0 {
                let parent_id: String = Serializable::read_from(buf)?;
                Some(parent_id)
            } else {
                None
            }
        };

        let has_display: u8 = Serializable::read_from(buf)?;
        let display_data = {
            if has_display != 0 {
                let display_data: AdvancementDisplay = Serializable::read_from(buf)?;
                Some(display_data)
            } else {
                None
            }
        };

        let criteria: LenPrefixed<VarInt, String> = Serializable::read_from(buf)?;
        let requirements: LenPrefixed<VarInt, LenPrefixed<VarInt, String>> =
            Serializable::read_from(buf)?;
        Ok(Advancement {
            id,
            parent_id,
            display_data,
            criteria,
            requirements,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.id.write_to(buf)?;
        self.parent_id.write_to(buf)?;
        self.display_data.write_to(buf)?;
        self.criteria.write_to(buf)?;
        self.requirements.write_to(buf)
    }
}

#[derive(Debug, Default)]
pub struct AdvancementDisplay {
    pub title: String,
    pub description: String,
    pub icon: Option<item::Stack>,
    pub frame_type: VarInt,
    pub flags: i32,
    pub background_texture: Option<String>,
    pub x_coord: f32,
    pub y_coord: f32,
}

impl Serializable for AdvancementDisplay {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        let title: String = Serializable::read_from(buf)?;
        let description: String = Serializable::read_from(buf)?;
        let icon: Option<item::Stack> = Serializable::read_from(buf)?;
        let frame_type: VarInt = Serializable::read_from(buf)?;
        let flags: i32 = Serializable::read_from(buf)?;
        let background_texture: Option<String> = if flags & 1 != 0 {
            Serializable::read_from(buf)?
        } else {
            None
        };
        let x_coord: f32 = Serializable::read_from(buf)?;
        let y_coord: f32 = Serializable::read_from(buf)?;

        Ok(AdvancementDisplay {
            title,
            description,
            icon,
            frame_type,
            flags,
            background_texture,
            x_coord,
            y_coord,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.title.write_to(buf)?;
        self.description.write_to(buf)?;
        self.icon.write_to(buf)?;
        self.frame_type.write_to(buf)?;
        self.flags.write_to(buf)?;
        if self.flags & 1 != 0 {
            self.background_texture.write_to(buf)?;
        }
        self.x_coord.write_to(buf)?;
        self.y_coord.write_to(buf)
    }
}

#[derive(Debug, Default)]
pub struct AdvancementProgress {
    pub id: String,
    pub criteria: LenPrefixed<VarInt, CriterionProgress>,
}

impl Serializable for AdvancementProgress {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        Ok(AdvancementProgress {
            id: Serializable::read_from(buf)?,
            criteria: Serializable::read_from(buf)?,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.id.write_to(buf)?;
        self.criteria.write_to(buf)
    }
}

#[derive(Debug, Default)]
pub struct CriterionProgress {
    pub id: String,
    pub date_of_achieving: Option<i64>,
}

impl Serializable for CriterionProgress {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error> {
        let id = Serializable::read_from(buf)?;
        let achieved: u8 = Serializable::read_from(buf)?;
        let date_of_achieving: Option<i64> = if achieved != 0 {
            Serializable::read_from(buf)?
        } else {
            None
        };

        Ok(CriterionProgress {
            id,
            date_of_achieving,
        })
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.id.write_to(buf)?;
        self.date_of_achieving.write_to(buf)
    }
}
