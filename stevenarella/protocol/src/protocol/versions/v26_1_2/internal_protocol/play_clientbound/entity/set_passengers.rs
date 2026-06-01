use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, LenPrefixed, Serializable, VarInt,
};

pub(super) fn read_set_passengers_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlaySetPassengersClientbound => {
            let vehicle_entity_id = VarInt::read_from(buf)?;
            let passenger_entity_ids: LenPrefixed<VarInt, VarInt> = LenPrefixed::read_from(buf)?;
            let passenger_ids: Vec<i32> = passenger_entity_ids.data.iter().map(|id| id.0).collect();
            if vehicle_entity_id.0 != 3 || passenger_ids != [4] {
                return Err(Error::Err(format!(
                    "unsupported Play set_passengers fixture vehicle {} passengers {:?}",
                    vehicle_entity_id.0, passenger_ids
                )));
            }
            Ok(Some(Packet::PlaySetPassengersClientbound(
                packet::play::clientbound::PlaySetPassengersClientbound {
                    vehicle_entity_id,
                    passenger_entity_ids,
                },
            )))
        }
        _ => Ok(None),
    }
}
