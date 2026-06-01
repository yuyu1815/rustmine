use crate::protocol::*;
use std::io;

#[derive(Default, Debug)]
pub struct ConfigurationClearDialogClientbound {
    pub empty: (),
}

impl PacketType for ConfigurationClearDialogClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            super::internal_ids::ConfigurationClearDialogClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.empty.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationShowDialogClientbound {
    pub dialog_data: Vec<u8>,
}

impl PacketType for ConfigurationShowDialogClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            super::internal_ids::ConfigurationShowDialogClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        buf.write_all(&self.dialog_data)?;
        Ok(())
    }
}
