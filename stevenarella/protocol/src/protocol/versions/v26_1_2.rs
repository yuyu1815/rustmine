protocol_packet_ids!(
    handshake Handshaking {
        serverbound Serverbound {
            0x00 => Handshake
        }
        clientbound Clientbound {
        }
    }
    configuration Configuration {
        serverbound Serverbound {
            0x00 => ConfigurationClientInformationServerbound
            0x01 => ConfigurationCookieResponseServerbound
            0x02 => ConfigurationCustomPayloadServerbound
            0x03 => ConfigurationFinishConfigurationServerbound
            0x04 => ConfigurationKeepAliveServerbound_i64
            0x05 => ConfigurationPongServerbound_i32
            0x06 => ConfigurationResourcePackServerbound
            0x07 => ConfigurationSelectKnownPacksServerbound
            0x08 => ConfigurationCustomClickActionServerbound
            0x09 => ConfigurationAcceptCodeOfConductServerbound
        }
        clientbound Clientbound {
            0x00 => ConfigurationCookieRequestClientbound
            0x01 => ConfigurationCustomPayloadClientbound
            0x02 => ConfigurationDisconnectClientbound
            0x03 => ConfigurationFinishConfigurationClientbound
            0x04 => ConfigurationKeepAliveClientbound_i64
            0x05 => ConfigurationPingClientbound_i32
            0x06 => ConfigurationResetChatClientbound
            0x07 => ConfigurationRegistryDataClientbound
            0x08 => ConfigurationResourcePackPopClientbound
            0x09 => ConfigurationResourcePackPushClientbound
            0x0a => ConfigurationStoreCookieClientbound
            0x0b => ConfigurationTransferClientbound
            0x0c => ConfigurationUpdateEnabledFeaturesClientbound
            0x0d => ConfigurationUpdateTagsClientbound
            0x0e => ConfigurationSelectKnownPacksClientbound
            0x0f => ConfigurationCustomReportDetailsClientbound
            0x10 => ConfigurationServerLinksClientbound
            0x11 => ConfigurationClearDialogClientbound
            0x12 => ConfigurationShowDialogClientbound
            0x13 => ConfigurationCodeOfConductClientbound
        }
    }
);
