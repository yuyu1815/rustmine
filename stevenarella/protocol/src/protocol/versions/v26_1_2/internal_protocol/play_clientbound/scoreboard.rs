use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable,
};

mod set_display_objective;
mod set_score;

pub(crate) fn read_scoreboard_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    if let Some(packet) =
        set_display_objective::read_set_display_objective_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        set_score::read_set_score_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }

    match internal_id {
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
