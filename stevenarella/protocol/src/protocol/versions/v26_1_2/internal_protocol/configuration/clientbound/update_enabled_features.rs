use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(super) fn read_update_enabled_features_configuration_clientbound_packet_by_internal_id<
    R: io::Read,
>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::clientbound::internal_ids::ConfigurationUpdateEnabledFeaturesClientbound => {
            let feature_count = VarInt::read_from(buf)?.0;
            if feature_count < 0 {
                return Err(Error::Err(format!(
                    "negative update_enabled_features feature count {}",
                    feature_count
                )));
            }
            let mut features = Vec::with_capacity(feature_count as usize);
            for _ in 0..feature_count {
                features.push(Serializable::read_from(buf)?);
            }
            let _packet = packet::configuration::clientbound::ConfigurationUpdateEnabledFeaturesClientbound {
                features,
            };
            Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "UpdateEnabledFeatures".to_owned(),
                    data: Vec::new(),
                },
            )))
        }
        _ => Ok(None),
    }
}
