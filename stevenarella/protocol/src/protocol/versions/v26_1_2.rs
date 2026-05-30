protocol_packet_ids!(
    configuration Configuration {
        serverbound Serverbound {
            0x03 => ConfigurationFinishConfigurationServerbound
            0x04 => ConfigurationKeepAliveServerbound_i64
            0x05 => ConfigurationPongServerbound_i32
        }
        clientbound Clientbound {
            0x03 => ConfigurationFinishConfigurationClientbound
            0x04 => ConfigurationKeepAliveClientbound_i64
            0x05 => ConfigurationPingClientbound_i32
        }
    }
);
