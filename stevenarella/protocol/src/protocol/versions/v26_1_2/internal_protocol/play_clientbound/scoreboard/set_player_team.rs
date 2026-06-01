use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable,
};

pub(super) fn read_set_player_team_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
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
