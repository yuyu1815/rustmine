use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(crate) fn read_custom_report_details_configuration_clientbound_packet_by_internal_id<
    R: io::Read,
>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::clientbound::internal_ids::ConfigurationCustomReportDetailsClientbound => {
            let detail_count = VarInt::read_from(buf)?.0;
            if detail_count < 0 {
                return Err(Error::Err(format!(
                    "negative custom_report_details detail count {}",
                    detail_count
                )));
            }
            let mut details = Vec::with_capacity(detail_count as usize);
            for _ in 0..detail_count {
                details.push(
                    packet::configuration::clientbound::ConfigurationCustomReportDetail {
                        key: Serializable::read_from(buf)?,
                        value: Serializable::read_from(buf)?,
                    },
                );
            }
            let _packet =
                packet::configuration::clientbound::ConfigurationCustomReportDetailsClientbound {
                    details,
                };
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "CustomReportDetails".to_owned(),
                    data: Vec::new(),
                },
            )))
        }
        _ => Ok(None),
    }
}
