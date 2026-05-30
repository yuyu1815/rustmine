protocol_packet_ids!(
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
            0x03 => ConfigurationFinishConfigurationClientbound
            0x04 => ConfigurationKeepAliveClientbound_i64
            0x05 => ConfigurationPingClientbound_i32
        }
    }
);
