use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(crate) fn read_scoreboard_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlaySetDisplayObjectiveClientbound => {
            let slot = VarInt::read_from(buf)?;
            let objective_name = String::read_from(buf)?;
            if !objective_name.is_empty() {
                return Err(Error::Err(format!(
                    "unsupported non-empty Play set_display_objective objective name {:?}",
                    objective_name
                )));
            }
            Ok(Some(Packet::PlaySetDisplayObjectiveClientbound(
                packet::play::clientbound::PlaySetDisplayObjectiveClientbound {
                    slot,
                    objective_name,
                },
            )))
        }
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
        packet::play::clientbound::internal_ids::PlaySetObjectiveClientbound => {
            let objective_name = String::read_from(buf)?;
            let method = i8::read_from(buf)?;
            if method != 1 {
                return Err(Error::Err(format!(
                    "unsupported Play set_objective method {}",
                    method
                )));
            }
            Ok(Some(Packet::PlaySetObjectiveClientbound(
                packet::play::clientbound::PlaySetObjectiveClientbound {
                    objective_name,
                    method,
                },
            )))
        }
        packet::play::clientbound::internal_ids::PlaySetPlayerTeamClientbound => {
            let team_name = String::read_from(buf)?;
            let method = i8::read_from(buf)?;
            if method != 1 {
                return Err(Error::Err(format!(
                    "unsupported Play set_player_team method {}",
                    method
                )));
            }
            Ok(Some(Packet::PlaySetPlayerTeamClientbound(
                packet::play::clientbound::PlaySetPlayerTeamClientbound { team_name, method },
            )))
        }
        _ => Ok(None),
    }
}
