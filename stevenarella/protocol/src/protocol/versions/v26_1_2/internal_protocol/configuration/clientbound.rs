use std::io;

use crate::protocol::{
    packet::{self, Packet},
    read_nbt_string_component, Direction, Error, LenPrefixed, Serializable, State, VarInt,
};

use super::super::super::translate_internal_packet_id;

mod cookie;
mod custom_payload;
mod custom_report_details;
mod dialog;
mod keep_alive_ping;
mod resource_pack;
mod server_links;
mod update;

pub(crate) fn read_configuration_clientbound_packet_by_id<R: io::Read>(
    id: i32,
    buf: &mut R,
) -> Result<Option<Packet>, Error> {
    let internal_id =
        translate_internal_packet_id(State::Configuration, Direction::Clientbound, id, true);
    if let Some(packet) =
        cookie::read_cookie_configuration_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        custom_payload::read_custom_payload_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        custom_report_details::read_custom_report_details_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        keep_alive_ping::read_keep_alive_ping_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        update::read_update_configuration_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        resource_pack::read_resource_pack_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        server_links::read_server_links_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        dialog::read_dialog_configuration_clientbound_packet_by_internal_id(internal_id, buf)?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        super::finish_configuration::read_finish_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }

    match internal_id {
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
        _ => Ok(None),
    }
}
