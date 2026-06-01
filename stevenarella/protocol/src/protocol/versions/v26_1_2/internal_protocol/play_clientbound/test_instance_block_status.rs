use std::io;

use crate::protocol::{
    packet::{self, Packet},
    read_nbt_string_component, Error, Serializable,
};

pub(crate) fn read_test_instance_block_status_play_clientbound_packet_by_internal_id<
    R: io::Read,
>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::play::clientbound::internal_ids::PlayTestInstanceBlockStatusClientbound => {
            let status = read_nbt_string_component(buf)?;
            let size_present = bool::read_from(buf)?;
            if size_present {
                return Err(Error::Err(
                    "unsupported Play test_instance_block_status present size".to_owned(),
                ));
            }
            Ok(Some(Packet::PlayTestInstanceBlockStatusClientbound(
                packet::play::clientbound::PlayTestInstanceBlockStatusClientbound {
                    status,
                    size_present,
                },
            )))
        }
        _ => Ok(None),
    }
}
