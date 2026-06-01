use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(super) fn read_teleport_entity_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlayTeleportEntityClientbound => {
            let entity_id = VarInt::read_from(buf)?;
            let position_x = buf.read_f64::<BigEndian>()?;
            let position_y = buf.read_f64::<BigEndian>()?;
            let position_z = buf.read_f64::<BigEndian>()?;
            let delta_x = buf.read_f64::<BigEndian>()?;
            let delta_y = buf.read_f64::<BigEndian>()?;
            let delta_z = buf.read_f64::<BigEndian>()?;
            let y_rot = buf.read_f32::<BigEndian>()?;
            let x_rot = buf.read_f32::<BigEndian>()?;
            let relative_mask = buf.read_i32::<BigEndian>()?;
            if relative_mask != 0 {
                return Err(Error::Err(format!(
                    "unsupported Play teleport_entity non-empty relative mask {}",
                    relative_mask
                )));
            }
            Ok(Some(Packet::PlayTeleportEntityClientbound(
                packet::play::clientbound::PlayTeleportEntityClientbound {
                    entity_id,
                    position_x,
                    position_y,
                    position_z,
                    delta_x,
                    delta_y,
                    delta_z,
                    y_rot,
                    x_rot,
                    relative_mask,
                    on_ground: bool::read_from(buf)?,
                },
            )))
        }
        _ => Ok(None),
    }
}
