use std::io;

use crate::protocol::{
    nbt,
    packet::{self, Packet},
    read_nbt_string_component, Direction, Error, LenPrefixed, Serializable, State, VarInt, UUID,
};

use super::super::translate_internal_packet_id;

pub(crate) fn read_configuration_packet_by_id<R: io::Read>(
    dir: Direction,
    id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    match dir {
        Direction::Serverbound => read_configuration_serverbound_packet_by_id(id, buf),
        Direction::Clientbound => read_configuration_clientbound_packet_by_id(id, buf),
    }
}

fn read_configuration_serverbound_packet_by_id<R: io::Read>(
    id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    let internal_id =
        translate_internal_packet_id(State::Configuration, Direction::Serverbound, id, true);
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
        packet::configuration::serverbound::internal_ids::ConfigurationCustomClickActionServerbound => {
            let id: String = Serializable::read_from(buf)?;
            let payload_len: VarInt = Serializable::read_from(buf)?;
            let payload = if payload_len.0 > 0 {
                let mut payload_bytes = vec![0; payload_len.0 as usize];
                io::Read::read_exact(buf, &mut payload_bytes)?;
                let mut payload_slice = payload_bytes.as_slice();
                let payload_type: u8 = Serializable::read_from(&mut payload_slice)?;
                if payload_type != 10 {
                    return Err(Error::Err("custom_click_action payload is not a compound tag".to_owned()));
                }
                Some(nbt::NamedTag(String::new(), nbt::Tag::read_from(&mut payload_slice)?))
            } else {
                None
            };
            let _packet = packet::configuration::serverbound::ConfigurationCustomClickActionServerbound {
                id,
                payload,
            };
            return Ok(Some(Packet::PluginMessageServerbound(
                packet::play::serverbound::PluginMessageServerbound {
                    channel: "CustomClickAction".to_owned(),
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

fn read_configuration_clientbound_packet_by_id<R: io::Read>(
    id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    let internal_id =
        translate_internal_packet_id(State::Configuration, Direction::Clientbound, id, true);
    match internal_id {
        packet::configuration::clientbound::internal_ids::ConfigurationCookieRequestClientbound => {
            let _packet = packet::configuration::clientbound::ConfigurationCookieRequestClientbound {
                key: Serializable::read_from(buf)?,
            };
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "CookieRequest".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationCustomPayloadClientbound => {
            let packet = packet::configuration::clientbound::ConfigurationCustomPayloadClientbound {
                channel: Serializable::read_from(buf)?,
                data: Serializable::read_from(buf)?,
            };
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: packet.channel,
                    data: packet.data,
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationDisconnectClientbound => {
            let packet = packet::configuration::clientbound::ConfigurationDisconnectClientbound {
                reason: read_nbt_string_component(buf)?,
            };
            return Ok(Some(Packet::Disconnect(
                packet::play::clientbound::Disconnect {
                    reason: packet.reason,
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationResetChatClientbound => {
            let _: () = Serializable::read_from(buf)?;
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "ResetChat".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationRegistryDataClientbound => {
            let _packet = packet::configuration::clientbound::ConfigurationRegistryDataClientbound {
                registry: Serializable::read_from(buf)?,
                data: Serializable::read_from(buf)?,
            };
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "RegistryData".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationResourcePackPopClientbound => {
            let id_present: bool = Serializable::read_from(buf)?;
            let _packet = packet::configuration::clientbound::ConfigurationResourcePackPopClientbound {
                id_present,
                id: if id_present {
                    Some(Serializable::read_from(buf)?)
                } else {
                    None
                },
            };
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "ResourcePackPop".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationResourcePackPushClientbound => {
            let id: UUID = Serializable::read_from(buf)?;
            let url: String = Serializable::read_from(buf)?;
            let hash: String = Serializable::read_from(buf)?;
            let required: bool = Serializable::read_from(buf)?;
            let prompt_present: bool = Serializable::read_from(buf)?;
            let _packet = packet::configuration::clientbound::ConfigurationResourcePackPushClientbound {
                id,
                url,
                hash,
                required,
                prompt_present,
                prompt_data: if prompt_present {
                    Serializable::read_from(buf)?
                } else {
                    Vec::new()
                },
            };
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "ResourcePackPush".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationStoreCookieClientbound => {
            let key: String = Serializable::read_from(buf)?;
            let payload_len = VarInt::read_from(buf)?.0;
            if payload_len < 0 {
                return Err(Error::Err(format!(
                    "negative store_cookie payload length {}",
                    payload_len
                )));
            }
            let mut payload = vec![0; payload_len as usize];
            buf.read_exact(&mut payload)?;
            let packet = packet::configuration::clientbound::ConfigurationStoreCookieClientbound {
                key,
                payload,
            };
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "StoreCookie".to_owned(),
                    data: packet.payload,
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationTransferClientbound => {
            let _packet = packet::configuration::clientbound::ConfigurationTransferClientbound {
                host: Serializable::read_from(buf)?,
                port: VarInt::read_from(buf)?.0,
            };
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "Transfer".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
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
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "UpdateEnabledFeatures".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationUpdateTagsClientbound => {
            let registry_payload_count = VarInt::read_from(buf)?.0;
            if registry_payload_count < 0 {
                return Err(Error::Err(format!(
                    "negative update_tags registry-payload count {}",
                    registry_payload_count
                )));
            }
            let mut registry_payloads =
                Vec::with_capacity(registry_payload_count as usize);
            for _ in 0..registry_payload_count {
                let registry_key = Serializable::read_from(buf)?;
                let tag_count = VarInt::read_from(buf)?.0;
                if tag_count < 0 {
                    return Err(Error::Err(format!(
                        "negative update_tags tag count {}",
                        tag_count
                    )));
                }
                let mut tags = Vec::with_capacity(tag_count as usize);
                for _ in 0..tag_count {
                    let tag_key = Serializable::read_from(buf)?;
                    let entry_count = VarInt::read_from(buf)?.0;
                    if entry_count < 0 {
                        return Err(Error::Err(format!(
                            "negative update_tags entry count {}",
                            entry_count
                        )));
                    }
                    let mut entry_ids = Vec::with_capacity(entry_count as usize);
                    for _ in 0..entry_count {
                        entry_ids.push(VarInt::read_from(buf)?.0);
                    }
                    tags.push(
                        packet::configuration::clientbound::ConfigurationUpdateTagsTagPayload {
                            tag_key,
                            entry_ids,
                        },
                    );
                }
                registry_payloads.push(
                    packet::configuration::clientbound::ConfigurationUpdateTagsRegistryPayload {
                        registry_key,
                        tags,
                    },
                );
            }
            let _packet = packet::configuration::clientbound::ConfigurationUpdateTagsClientbound {
                registry_payloads,
            };
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "UpdateTags".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
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
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "SelectKnownPacks".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationCustomReportDetailsClientbound => {
            let detail_count = VarInt::read_from(buf)?.0;
            if detail_count < 0 {
                return Err(Error::Err(format!(
                    "negative custom_report_details detail count {}",
                    detail_count
                )));
            }
            let mut details = Vec::with_capacity(detail_count as usize);
            for _ in 0..detail_count {
                details.push(
                    packet::configuration::clientbound::ConfigurationCustomReportDetail {
                        key: Serializable::read_from(buf)?,
                        value: Serializable::read_from(buf)?,
                    },
                );
            }
            let _packet = packet::configuration::clientbound::ConfigurationCustomReportDetailsClientbound {
                details,
            };
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "CustomReportDetails".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationServerLinksClientbound => {
            let link_count = VarInt::read_from(buf)?.0;
            if link_count < 0 {
                return Err(Error::Err(format!(
                    "negative server_links link count {}",
                    link_count
                )));
            }
            if link_count != 0 {
                return Err(Error::Err(format!(
                    "unsupported non-empty server_links list count {}",
                    link_count
                )));
            }
            let _packet = packet::configuration::clientbound::ConfigurationServerLinksClientbound {
                link_count,
                links_data: Vec::new(),
            };
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "ServerLinks".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationClearDialogClientbound => {
            let _packet = packet::configuration::clientbound::ConfigurationClearDialogClientbound {
                empty: Serializable::read_from(buf)?,
            };
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "ClearDialog".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationShowDialogClientbound => {
            let mut dialog_data = Vec::new();
            buf.read_to_end(&mut dialog_data)?;
            let packet =
                packet::configuration::clientbound::ConfigurationShowDialogClientbound {
                    dialog_data,
                };
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "ShowDialog".to_owned(),
                    data: packet.dialog_data,
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationCodeOfConductClientbound => {
            let packet =
                packet::configuration::clientbound::ConfigurationCodeOfConductClientbound {
                    code_of_conduct: Serializable::read_from(buf)?,
                };
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "CodeOfConduct".to_owned(),
                    data: packet.code_of_conduct.into_bytes(),
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationFinishConfigurationClientbound => {
            let _: () = Serializable::read_from(buf)?;
            return Ok(Some(Packet::PluginMessageClientbound(
                packet::play::clientbound::PluginMessageClientbound {
                    channel: "FinishConfiguration".to_owned(),
                    data: Vec::new(),
                },
            )));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationKeepAliveClientbound_i64 => {
            let mut packet = packet::play::clientbound::KeepAliveClientbound_i64::default();
            packet.id = Serializable::read_from(buf)?;
            return Ok(Some(Packet::KeepAliveClientbound_i64(packet)));
        }
        packet::configuration::clientbound::internal_ids::ConfigurationPingClientbound_i32 => {
            let id: i32 = Serializable::read_from(buf)?;
            return Ok(Some(Packet::StatusPing(
                packet::status::serverbound::StatusPing { ping: id.into() },
            )));
        }

        _ => Ok(None),
    }
}
