use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, LenPrefixed, Serializable, VarInt,
};

pub(crate) fn read_select_known_packs_configuration_clientbound_packet_by_internal_id<
    R: io::Read,
>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::clientbound::internal_ids::ConfigurationSelectKnownPacksClientbound => {
            let known_pack_count = VarInt::read_from(buf)?.0;
            if known_pack_count < 0 {
                return Err(Error::Err(format!(
                    "negative clientbound select_known_packs known-pack count {}",
                    known_pack_count
                )));
            }
            let mut known_packs = Vec::with_capacity(known_pack_count as usize);
            for _ in 0..known_pack_count {
                known_packs.push(Serializable::read_from(buf)?);
            }
            let _packet = packet::configuration::clientbound::ConfigurationSelectKnownPacksClientbound {
                known_packs: LenPrefixed::new(known_packs),
            };
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "SelectKnownPacks".to_owned(),
                    data: Vec::new(),
                },
            )))
        }
        _ => Ok(None),
    }
}
