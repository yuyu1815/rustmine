use crate::protocol::*;
use std::io;

#[derive(Default, Debug)]
pub struct ConfigurationCustomReportDetail {
    pub key: String,
    pub value: String,
}

#[derive(Default, Debug)]
pub struct ConfigurationCustomReportDetailsClientbound {
    pub details: Vec<ConfigurationCustomReportDetail>,
}

impl PacketType for ConfigurationCustomReportDetailsClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            super::internal_ids::ConfigurationCustomReportDetailsClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        VarInt(self.details.len() as i32).write_to(buf)?;
        for detail in &self.details {
            detail.key.write_to(buf)?;
            detail.value.write_to(buf)?;
        }
        Ok(())
    }
}
