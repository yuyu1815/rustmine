use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(crate) fn read_custom_report_details_play_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlayCustomReportDetailsClientbound => {
            let detail_count = VarInt::read_from(buf)?;
            if detail_count.0 < 0 {
                return Err(Error::Err(format!(
                    "negative Play custom_report_details detail count {}",
                    detail_count.0
                )));
            }
            if detail_count.0 != 0 {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play custom_report_details map count {}",
                    detail_count.0
                )));
            }
            Ok(Some(Packet::PlayCustomReportDetailsClientbound(
                packet::play::clientbound::PlayCustomReportDetailsClientbound { detail_count },
            )))
        }
        _ => Ok(None),
    }
}
