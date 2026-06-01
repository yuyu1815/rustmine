use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(crate) fn read_waypoint_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlayWaypointClientbound => {
            let operation_id = VarInt::read_from(buf)?;
            if operation_id.0 != 1 {
                return Err(Error::Err(format!(
                    "unsupported Play waypoint operation id {}",
                    operation_id.0
                )));
            }
            let mut waypoint_payload = Vec::new();
            buf.read_to_end(&mut waypoint_payload)?;
            let expected_payload = [
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x01, 0x23, 0x11, 0x6d, 0x69, 0x6e, 0x65, 0x63, 0x72, 0x61, 0x66, 0x74, 0x3a,
                0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x00, 0x00,
            ];
            if waypoint_payload != expected_payload {
                return Err(Error::Err(
                    "unsupported Play waypoint payload outside removeWaypoint empty fixture"
                        .to_owned(),
                ));
            }
            Ok(Some(Packet::PlayWaypointClientbound(
                packet::play::clientbound::PlayWaypointClientbound {
                    operation_id,
                    waypoint_payload,
                },
            )))
        }
        _ => Ok(None),
    }
}
