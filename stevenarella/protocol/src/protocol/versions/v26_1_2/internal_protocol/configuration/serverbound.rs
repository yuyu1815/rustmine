use std::io;

use crate::protocol::{
    packet::{self, Packet},
    Direction, Error, Serializable, State, VarInt, UUID,
};

use super::super::super::translate_internal_packet_id;

mod custom_click_action;

pub(crate) fn read_configuration_serverbound_packet_by_id<R: io::Read>(
    id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    let internal_id =
        translate_internal_packet_id(State::Configuration, Direction::Serverbound, id, true);
    if let Some(packet) =
        custom_click_action::read_custom_click_action_configuration_serverbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }

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
            return Ok(Some(Packet::PluginMessageServerbound(
                packet::play::serverbound::PluginMessageServerbound {
                    channel: "ClientInformation".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
        packet::configuration::serverbound::internal_ids::ConfigurationCookieResponseServerbound => {
            let packet = packet::configuration::serverbound::ConfigurationCookieResponseServerbound {
                key: Serializable::read_from(buf)?,
                payload: if bool::read_from(buf)? {
                    Some(Serializable::read_from(buf)?)
                } else {
                    None
                },
            };
            return Ok(Some(Packet::PluginMessageServerbound(
                packet::play::serverbound::PluginMessageServerbound {
                    channel: "CookieResponse".to_owned(),
                    data: packet
                        .payload
                        .map(|payload| payload.data)
                        .unwrap_or_else(Vec::new),
                },
            )));
        }
        packet::configuration::serverbound::internal_ids::ConfigurationCustomPayloadServerbound => {
            let packet = packet::configuration::serverbound::ConfigurationCustomPayloadServerbound {
                channel: Serializable::read_from(buf)?,
                data: Serializable::read_from(buf)?,
            };
            return Ok(Some(Packet::PluginMessageServerbound(
                packet::play::serverbound::PluginMessageServerbound {
                    channel: packet.channel,
                    data: packet.data,
                },
            )));
        }
        packet::configuration::serverbound::internal_ids::ConfigurationKeepAliveServerbound_i64 => {
            let mut packet = packet::play::serverbound::KeepAliveServerbound_i64::default();
            packet.id = Serializable::read_from(buf)?;
            return Ok(Some(Packet::KeepAliveServerbound_i64(packet)));
        }
        packet::configuration::serverbound::internal_ids::ConfigurationPongServerbound_i32 => {
            let id: i32 = Serializable::read_from(buf)?;
            return Ok(Some(Packet::StatusPong(
                packet::status::clientbound::StatusPong { ping: id.into() },
            )));
        }
        packet::configuration::serverbound::internal_ids::ConfigurationFinishConfigurationServerbound => {
            let _: () = Serializable::read_from(buf)?;
            return Ok(Some(Packet::PluginMessageServerbound(
                packet::play::serverbound::PluginMessageServerbound {
                    channel: "FinishConfiguration".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
        packet::configuration::serverbound::internal_ids::ConfigurationResourcePackServerbound => {
            let _id: UUID = Serializable::read_from(buf)?;
            let action: VarInt = Serializable::read_from(buf)?;
            return Ok(Some(Packet::ResourcePackStatus(
                packet::play::serverbound::ResourcePackStatus { result: action },
            )));
        }
        packet::configuration::serverbound::internal_ids::ConfigurationSelectKnownPacksServerbound => {
            let _packet = packet::configuration::serverbound::ConfigurationSelectKnownPacksServerbound {
                known_packs: Serializable::read_from(buf)?,
            };
            return Ok(Some(Packet::PluginMessageServerbound(
                packet::play::serverbound::PluginMessageServerbound {
                    channel: "SelectKnownPacks".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
        packet::configuration::serverbound::internal_ids::ConfigurationAcceptCodeOfConductServerbound => {
            let _: () = Serializable::read_from(buf)?;
            return Ok(Some(Packet::PluginMessageServerbound(
                packet::play::serverbound::PluginMessageServerbound {
                    channel: "AcceptCodeOfConduct".to_owned(),
                    data: Vec::new(),
                },
            )));
        }

        _ => Ok(None),
    }
}
