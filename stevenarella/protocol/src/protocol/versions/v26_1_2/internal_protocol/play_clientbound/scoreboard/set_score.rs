use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(super) fn read_set_score_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlaySetScoreClientbound => {
            let owner = String::read_from(buf)?;
            let objective_name = String::read_from(buf)?;
            let score = VarInt::read_from(buf)?;
            let display_present = bool::read_from(buf)?;
            if display_present {
                return Err(Error::Err(
                    "unsupported Play set_score optional display Component".to_owned(),
                ));
            }
            let number_format_present = bool::read_from(buf)?;
            if number_format_present {
                return Err(Error::Err(
                    "unsupported Play set_score optional number format".to_owned(),
                ));
            }
            Ok(Some(Packet::PlaySetScoreClientbound(
                packet::play::clientbound::PlaySetScoreClientbound {
                    owner,
                    objective_name,
                    score,
                    display_present,
                    number_format_present,
                },
            )))
        }
        _ => Ok(None),
    }
}
