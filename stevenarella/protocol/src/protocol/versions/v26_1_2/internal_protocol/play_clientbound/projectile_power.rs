use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(super) fn read_projectile_power_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlayProjectilePowerClientbound => {
            Ok(Some(Packet::PlayProjectilePowerClientbound(
                packet::play::clientbound::PlayProjectilePowerClientbound {
                    entity_id: VarInt::read_from(buf)?,
                    acceleration_power: buf.read_f64::<BigEndian>()?,
                },
            )))
        }
        _ => Ok(None),
    }
}
