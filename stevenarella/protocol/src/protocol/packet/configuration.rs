pub mod serverbound {
    use crate::nbt;
    use crate::protocol::*;
    use std::io;

    #[allow(non_upper_case_globals)]
    pub mod internal_ids {
        pub const ConfigurationFinishConfigurationServerbound: i32 = 0;
        pub const ConfigurationKeepAliveServerbound_i64: i32 = 1;
        pub const ConfigurationPongServerbound_i32: i32 = 2;
        pub const ConfigurationClientInformationServerbound: i32 = 3;
        pub const ConfigurationResourcePackServerbound: i32 = 4;
        pub const ConfigurationSelectKnownPacksServerbound: i32 = 5;
        pub const ConfigurationCustomClickActionServerbound: i32 = 6;
        pub const ConfigurationAcceptCodeOfConductServerbound: i32 = 7;
        pub const ConfigurationCookieResponseServerbound: i32 = 8;
        pub const ConfigurationCustomPayloadServerbound: i32 = 9;
    }

    #[derive(Default, Debug)]
    pub struct KnownPack {
        pub namespace: String,
        pub id: String,
        pub version: String,
    }

    impl Serializable for KnownPack {
        fn read_from<R: io::Read>(buf: &mut R) -> Result<KnownPack, Error> {
            Ok(KnownPack {
                namespace: Serializable::read_from(buf)?,
                id: Serializable::read_from(buf)?,
                version: Serializable::read_from(buf)?,
            })
        }

        fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
            self.namespace.write_to(buf)?;
            self.id.write_to(buf)?;
            self.version.write_to(buf)?;
            Ok(())
        }
    }

    #[derive(Default, Debug)]
    pub struct ConfigurationClientInformationServerbound {
        pub language: String,
        pub view_distance: u8,
        pub chat_visibility: VarInt,
        pub chat_colors: bool,
        pub model_customisation: u8,
        pub main_hand: VarInt,
        pub text_filtering_enabled: bool,
        pub allows_listing: bool,
        pub particle_status: VarInt,
    }

    impl PacketType for ConfigurationClientInformationServerbound {
        fn packet_id(&self, version: i32) -> i32 {
            packet::versions::translate_internal_packet_id_for_version(
                version,
                State::Configuration,
                Direction::Serverbound,
                internal_ids::ConfigurationClientInformationServerbound,
                false,
            )
        }

        fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
            self.language.write_to(buf)?;
            self.view_distance.write_to(buf)?;
            self.chat_visibility.write_to(buf)?;
            self.chat_colors.write_to(buf)?;
            self.model_customisation.write_to(buf)?;
            self.main_hand.write_to(buf)?;
            self.text_filtering_enabled.write_to(buf)?;
            self.allows_listing.write_to(buf)?;
            self.particle_status.write_to(buf)?;
            Ok(())
        }
    }

    #[derive(Default, Debug)]
    pub struct ConfigurationCookieResponseServerbound {
        pub key: String,
        pub payload: Option<LenPrefixedBytes<VarInt>>,
    }

    impl PacketType for ConfigurationCookieResponseServerbound {
        fn packet_id(&self, version: i32) -> i32 {
            packet::versions::translate_internal_packet_id_for_version(
                version,
                State::Configuration,
                Direction::Serverbound,
                internal_ids::ConfigurationCookieResponseServerbound,
                false,
            )
        }

        fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
            self.key.write_to(buf)?;
            self.payload.is_some().write_to(buf)?;
            if let Some(ref payload) = self.payload {
                payload.write_to(buf)?;
            }
            Ok(())
        }
    }

    #[derive(Default, Debug)]
    pub struct ConfigurationCustomPayloadServerbound {
        pub channel: String,
        pub data: Vec<u8>,
    }

    impl PacketType for ConfigurationCustomPayloadServerbound {
        fn packet_id(&self, version: i32) -> i32 {
            packet::versions::translate_internal_packet_id_for_version(
                version,
                State::Configuration,
                Direction::Serverbound,
                internal_ids::ConfigurationCustomPayloadServerbound,
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
    pub struct ConfigurationFinishConfigurationServerbound {
        pub empty: (),
    }

    impl PacketType for ConfigurationFinishConfigurationServerbound {
        fn packet_id(&self, version: i32) -> i32 {
            packet::versions::translate_internal_packet_id_for_version(
                version,
                State::Configuration,
                Direction::Serverbound,
                internal_ids::ConfigurationFinishConfigurationServerbound,
                false,
            )
        }

        fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
            self.empty.write_to(buf)?;
            Ok(())
        }
    }

    #[derive(Default, Debug)]
    pub struct ConfigurationKeepAliveServerbound_i64 {
        pub id: i64,
    }

    impl PacketType for ConfigurationKeepAliveServerbound_i64 {
        fn packet_id(&self, version: i32) -> i32 {
            packet::versions::translate_internal_packet_id_for_version(
                version,
                State::Configuration,
                Direction::Serverbound,
                internal_ids::ConfigurationKeepAliveServerbound_i64,
                false,
            )
        }

        fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
            self.id.write_to(buf)?;
            Ok(())
        }
    }

    #[derive(Default, Debug)]
    pub struct ConfigurationPongServerbound_i32 {
        pub id: i32,
    }

    impl PacketType for ConfigurationPongServerbound_i32 {
        fn packet_id(&self, version: i32) -> i32 {
            packet::versions::translate_internal_packet_id_for_version(
                version,
                State::Configuration,
                Direction::Serverbound,
                internal_ids::ConfigurationPongServerbound_i32,
                false,
            )
        }

        fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
            self.id.write_to(buf)?;
            Ok(())
        }
    }

    #[derive(Default, Debug)]
    pub struct ConfigurationResourcePackServerbound {
        pub id: UUID,
        pub action: VarInt,
    }

    impl PacketType for ConfigurationResourcePackServerbound {
        fn packet_id(&self, version: i32) -> i32 {
            packet::versions::translate_internal_packet_id_for_version(
                version,
                State::Configuration,
                Direction::Serverbound,
                internal_ids::ConfigurationResourcePackServerbound,
                false,
            )
        }

        fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
            self.id.write_to(buf)?;
            self.action.write_to(buf)?;
            Ok(())
        }
    }

    #[derive(Default, Debug)]
    pub struct ConfigurationSelectKnownPacksServerbound {
        pub known_packs: LenPrefixed<VarInt, KnownPack>,
    }

    impl PacketType for ConfigurationSelectKnownPacksServerbound {
        fn packet_id(&self, version: i32) -> i32 {
            packet::versions::translate_internal_packet_id_for_version(
                version,
                State::Configuration,
                Direction::Serverbound,
                internal_ids::ConfigurationSelectKnownPacksServerbound,
                false,
            )
        }

        fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
            self.known_packs.write_to(buf)?;
            Ok(())
        }
    }

    #[derive(Default, Debug)]
    pub struct ConfigurationCustomClickActionServerbound {
        pub id: String,
        pub payload: Option<nbt::NamedTag>,
    }

    impl PacketType for ConfigurationCustomClickActionServerbound {
        fn packet_id(&self, version: i32) -> i32 {
            packet::versions::translate_internal_packet_id_for_version(
                version,
                State::Configuration,
                Direction::Serverbound,
                internal_ids::ConfigurationCustomClickActionServerbound,
                false,
            )
        }

        fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
            self.id.write_to(buf)?;
            match self.payload {
                Some(ref payload) => {
                    let mut payload_bytes = Vec::new();
                    10u8.write_to(&mut payload_bytes)?;
                    payload.1.write_to(&mut payload_bytes)?;
                    VarInt(payload_bytes.len() as i32).write_to(buf)?;
                    buf.write_all(&payload_bytes)?;
                }
                None => VarInt(0).write_to(buf)?,
            }
            Ok(())
        }
    }

    #[derive(Default, Debug)]
    pub struct ConfigurationAcceptCodeOfConductServerbound {
        pub empty: (),
    }

    impl PacketType for ConfigurationAcceptCodeOfConductServerbound {
        fn packet_id(&self, version: i32) -> i32 {
            packet::versions::translate_internal_packet_id_for_version(
                version,
                State::Configuration,
                Direction::Serverbound,
                internal_ids::ConfigurationAcceptCodeOfConductServerbound,
                false,
            )
        }

        fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
            self.empty.write_to(buf)?;
            Ok(())
        }
    }
}

pub mod clientbound {
    use crate::protocol::*;
    use std::io;

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
    pub struct ConfigurationResourcePackPopClientbound {
        pub id_present: bool,
        pub id: Option<UUID>,
    }

    impl PacketType for ConfigurationResourcePackPopClientbound {
        fn packet_id(&self, version: i32) -> i32 {
            packet::versions::translate_internal_packet_id_for_version(
                version,
                State::Configuration,
                Direction::Clientbound,
                internal_ids::ConfigurationResourcePackPopClientbound,
                false,
            )
        }

        fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
            self.id_present.write_to(buf)?;
            if let Some(ref id) = self.id {
                id.write_to(buf)?;
            }
            Ok(())
        }
    }

    #[derive(Default, Debug)]
    pub struct ConfigurationResourcePackPushClientbound {
        pub id: UUID,
        pub url: String,
        pub hash: String,
        pub required: bool,
        pub prompt_present: bool,
        pub prompt_data: Vec<u8>,
    }

    impl PacketType for ConfigurationResourcePackPushClientbound {
        fn packet_id(&self, version: i32) -> i32 {
            packet::versions::translate_internal_packet_id_for_version(
                version,
                State::Configuration,
                Direction::Clientbound,
                internal_ids::ConfigurationResourcePackPushClientbound,
                false,
            )
        }

        fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
            self.id.write_to(buf)?;
            self.url.write_to(buf)?;
            self.hash.write_to(buf)?;
            self.required.write_to(buf)?;
            self.prompt_present.write_to(buf)?;
            self.prompt_data.write_to(buf)?;
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
    pub struct ConfigurationUpdateTagsTagPayload {
        pub tag_key: String,
        pub entry_ids: Vec<i32>,
    }

    #[derive(Default, Debug)]
    pub struct ConfigurationUpdateTagsRegistryPayload {
        pub registry_key: String,
        pub tags: Vec<ConfigurationUpdateTagsTagPayload>,
    }

    #[derive(Default, Debug)]
    pub struct ConfigurationUpdateTagsClientbound {
        pub registry_payloads: Vec<ConfigurationUpdateTagsRegistryPayload>,
    }

    impl PacketType for ConfigurationUpdateTagsClientbound {
        fn packet_id(&self, version: i32) -> i32 {
            packet::versions::translate_internal_packet_id_for_version(
                version,
                State::Configuration,
                Direction::Clientbound,
                internal_ids::ConfigurationUpdateTagsClientbound,
                false,
            )
        }

        fn write<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
            VarInt(self.registry_payloads.len() as i32).write_to(buf)?;
            for registry in &self.registry_payloads {
                registry.registry_key.write_to(buf)?;
                VarInt(registry.tags.len() as i32).write_to(buf)?;
                for tag in &registry.tags {
                    tag.tag_key.write_to(buf)?;
                    VarInt(tag.entry_ids.len() as i32).write_to(buf)?;
                    for entry_id in &tag.entry_ids {
                        VarInt(*entry_id).write_to(buf)?;
                    }
                }
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
}
