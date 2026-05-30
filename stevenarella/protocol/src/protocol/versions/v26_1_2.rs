protocol_packet_ids!(
    configuration Configuration {
        serverbound Serverbound {
            0x03 => ConfigurationFinishConfigurationServerbound
            0x04 => ConfigurationKeepAliveServerbound_i64
        }
        clientbound Clientbound {
            0x03 => ConfigurationFinishConfigurationClientbound
            0x04 => ConfigurationKeepAliveClientbound_i64
        }
    }
);
