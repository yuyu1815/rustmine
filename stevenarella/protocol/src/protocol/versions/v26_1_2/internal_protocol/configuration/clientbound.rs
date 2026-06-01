use std::io;

use crate::protocol::{
    packet::{self, Packet},
    read_nbt_string_component, Direction, Error, Serializable, State,
};

use super::super::super::translate_internal_packet_id;

mod code_of_conduct;
mod cookie;
mod custom_payload;
mod custom_report_details;
mod dialog;
mod keep_alive_ping;
mod resource_pack;
mod select_known_packs;
mod server_links;
mod transfer;
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
    if let Some(packet) =
        code_of_conduct::read_code_of_conduct_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        select_known_packs::read_select_known_packs_configuration_clientbound_packet_by_internal_id(
            internal_id,
            buf,
        )?
    {
        return Ok(Some(packet));
    }
    if let Some(packet) =
        transfer::read_transfer_configuration_clientbound_packet_by_internal_id(internal_id, buf)?
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
            let _packet =
                packet::configuration::clientbound::ConfigurationRegistryDataClientbound {
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
        _ => Ok(None),
    }
}
