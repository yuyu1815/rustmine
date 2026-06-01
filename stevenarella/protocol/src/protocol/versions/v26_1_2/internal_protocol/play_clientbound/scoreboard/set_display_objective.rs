use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(super) fn read_set_display_objective_clientbound_packet_by_internal_id<R: io::Read>(
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
        _ => Ok(None),
    }
}
