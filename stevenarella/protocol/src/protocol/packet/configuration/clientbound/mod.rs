use crate::protocol::*;
use std::io;

mod resource_pack;
mod update_tags;

pub use resource_pack::{
    ConfigurationResourcePackPopClientbound, ConfigurationResourcePackPushClientbound,
};
pub use update_tags::{
    ConfigurationUpdateTagsClientbound, ConfigurationUpdateTagsRegistryPayload,
    ConfigurationUpdateTagsTagPayload,
};

#[allow(non_upper_case_globals)]
pub mod internal_ids {
    pub const ConfigurationCookieRequestClientbound: i32 = 3;
    pub const ConfigurationCustomPayloadClientbound: i32 = 4;
    pub const ConfigurationDisconnectClientbound: i32 = 5;
    pub const ConfigurationFinishConfigurationClientbound: i32 = 0;
    pub const ConfigurationKeepAliveClientbound_i64: i32 = 1;
    pub const ConfigurationPingClientbound_i32: i32 = 2;
    pub const ConfigurationResetChatClientbound: i32 = 6;
    pub const ConfigurationRegistryDataClientbound: i32 = 7;
    pub const ConfigurationResourcePackPopClientbound: i32 = 8;
    pub const ConfigurationResourcePackPushClientbound: i32 = 9;
    pub const ConfigurationStoreCookieClientbound: i32 = 10;
    pub const ConfigurationTransferClientbound: i32 = 11;
    pub const ConfigurationUpdateEnabledFeaturesClientbound: i32 = 12;
    pub const ConfigurationUpdateTagsClientbound: i32 = 13;
    pub const ConfigurationSelectKnownPacksClientbound: i32 = 14;
    pub const ConfigurationCustomReportDetailsClientbound: i32 = 15;
    pub const ConfigurationServerLinksClientbound: i32 = 16;
    pub const ConfigurationClearDialogClientbound: i32 = 17;
    pub const ConfigurationShowDialogClientbound: i32 = 18;
    pub const ConfigurationCodeOfConductClientbound: i32 = 19;
}

#[derive(Default, Debug)]
pub struct ConfigurationCookieRequestClientbound {
    pub key: String,
}

impl PacketType for ConfigurationCookieRequestClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationCookieRequestClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.key.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationCustomPayloadClientbound {
    pub channel: String,
    pub data: Vec<u8>,
}

impl PacketType for ConfigurationCustomPayloadClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationCustomPayloadClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.channel.write_to(buf)?;
        self.data.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationDisconnectClientbound {
    pub reason: format::Component,
}

impl PacketType for ConfigurationDisconnectClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationDisconnectClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.reason.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationResetChatClientbound {
    pub empty: (),
}

impl PacketType for ConfigurationResetChatClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationResetChatClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.empty.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationRegistryDataClientbound {
    pub registry: String,
    pub data: Vec<u8>,
}

impl PacketType for ConfigurationRegistryDataClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationRegistryDataClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.registry.write_to(buf)?;
        self.data.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationStoreCookieClientbound {
    pub key: String,
    pub payload: Vec<u8>,
}

impl PacketType for ConfigurationStoreCookieClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationStoreCookieClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.key.write_to(buf)?;
        VarInt(self.payload.len() as i32).write_to(buf)?;
        buf.write_all(&self.payload)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationTransferClientbound {
    pub host: String,
    pub port: i32,
}

impl PacketType for ConfigurationTransferClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationTransferClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.host.write_to(buf)?;
        VarInt(self.port).write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationUpdateEnabledFeaturesClientbound {
    pub features: Vec<String>,
}

impl PacketType for ConfigurationUpdateEnabledFeaturesClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationUpdateEnabledFeaturesClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        VarInt(self.features.len() as i32).write_to(buf)?;
        for feature in &self.features {
            feature.write_to(buf)?;
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationSelectKnownPacksClientbound {
    pub known_packs: LenPrefixed<VarInt, super::serverbound::KnownPack>,
}

impl PacketType for ConfigurationSelectKnownPacksClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationSelectKnownPacksClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.known_packs.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationCustomReportDetail {
    pub key: String,
    pub value: String,
}

#[derive(Default, Debug)]
pub struct ConfigurationCustomReportDetailsClientbound {
    pub details: Vec<ConfigurationCustomReportDetail>,
}

impl PacketType for ConfigurationCustomReportDetailsClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationCustomReportDetailsClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        VarInt(self.details.len() as i32).write_to(buf)?;
        for detail in &self.details {
            detail.key.write_to(buf)?;
            detail.value.write_to(buf)?;
        }
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationServerLinksClientbound {
    pub link_count: i32,
    pub links_data: Vec<u8>,
}

impl PacketType for ConfigurationServerLinksClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationServerLinksClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        VarInt(self.link_count).write_to(buf)?;
        buf.write_all(&self.links_data)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationClearDialogClientbound {
    pub empty: (),
}

impl PacketType for ConfigurationClearDialogClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationClearDialogClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.empty.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationShowDialogClientbound {
    pub dialog_data: Vec<u8>,
}

impl PacketType for ConfigurationShowDialogClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationShowDialogClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        buf.write_all(&self.dialog_data)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationCodeOfConductClientbound {
    pub code_of_conduct: String,
}

impl PacketType for ConfigurationCodeOfConductClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationCodeOfConductClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.code_of_conduct.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationFinishConfigurationClientbound {
    pub empty: (),
}

impl PacketType for ConfigurationFinishConfigurationClientbound {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationFinishConfigurationClientbound,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.empty.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationKeepAliveClientbound_i64 {
    pub id: i64,
}

impl PacketType for ConfigurationKeepAliveClientbound_i64 {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationKeepAliveClientbound_i64,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.id.write_to(buf)?;
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ConfigurationPingClientbound_i32 {
    pub id: i32,
}

impl PacketType for ConfigurationPingClientbound_i32 {
    fn packet_id(&self, version: i32) -> i32 {
        packet::versions::translate_internal_packet_id_for_version(
            version,
            State::Configuration,
            Direction::Clientbound,
            internal_ids::ConfigurationPingClientbound_i32,
            false,
        )
    }

    fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        self.id.write_to(buf)?;
        Ok(())
    }
}
