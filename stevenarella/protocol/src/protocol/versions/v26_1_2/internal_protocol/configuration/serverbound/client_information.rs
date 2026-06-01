use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Error, Serializable, VarInt,
};

pub(crate) fn read_client_information_configuration_serverbound_packet_by_internal_id<
    R: io::Read,
>(
    internal_id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match internal_id {
        packet::configuration::serverbound::internal_ids::ConfigurationClientInformationServerbound => {
            let _language: String = Serializable::read_from(buf)?;
            let _view_distance: u8 = Serializable::read_from(buf)?;
            let _chat_visibility: VarInt = Serializable::read_from(buf)?;
            let _chat_colors: bool = Serializable::read_from(buf)?;
            let _model_customisation: u8 = Serializable::read_from(buf)?;
            let _main_hand: VarInt = Serializable::read_from(buf)?;
            let _text_filtering_enabled: bool = Serializable::read_from(buf)?;
            let _allows_listing: bool = Serializable::read_from(buf)?;
            let _particle_status: VarInt = Serializable::read_from(buf)?;
            Ok(Some(Packet::PluginMessageServerbound(
                packet::play::serverbound::PluginMessageServerbound {
                    channel: "ClientInformation".to_owned(),
                    data: Vec::new(),
                },
            )))
        }
        _ => Ok(None),
    }
}
