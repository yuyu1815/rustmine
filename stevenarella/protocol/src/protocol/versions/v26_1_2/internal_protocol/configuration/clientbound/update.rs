use std::io;

use crate::protocol::{packet::Packet, Error};

pub(crate) fn read_update_configuration_clientbound_packet_by_internal_id<R: io::Read>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    if let Some(packet) =
        super::update_enabled_features::read_update_enabled_features_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        super::update_tags::read_update_tags_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }

    Ok(None)
}
