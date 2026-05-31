protocol_packet_ids!(
    handshake Handshaking {
        serverbound Serverbound {
            0x00 => Handshake
        }
        clientbound Clientbound {
        }
    }
    login Login {
        serverbound Serverbound {
            0x00 => LoginStart
            0x01 => EncryptionResponse
            0x02 => LoginPluginResponse
            0x03 => LoginAcknowledged
            0x04 => LoginCookieResponse
        }
        clientbound Clientbound {
            0x00 => LoginDisconnect
            0x01 => EncryptionRequest_ShouldAuthenticate
            0x02 => LoginSuccess_UUID
            0x03 => SetInitialCompression
            0x04 => LoginPluginRequest
            0x05 => LoginCookieRequest
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
    play Play {
        serverbound Serverbound {
        }
        clientbound Clientbound {
            0x00 => BundleDelimiterClientbound
            0x01 => PlayAddEntityClientbound
            0x02 => PlayAnimateClientbound
            0x03 => PlayAwardStatsClientbound
            0x04 => PlayBlockChangedAckClientbound
            0x05 => PlayBlockDestructionClientbound
            0x06 => PlayBlockEntityDataClientbound
            0x07 => PlayBlockEventClientbound
            0x08 => PlayBlockUpdateClientbound
            0x09 => BossBar
            0x0a => ServerDifficulty_Locked
            0x0b => PlayChunkBatchFinishedClientbound
        }
    }
);
