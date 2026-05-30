protocol_packet_ids!(
    configuration Configuration {
        serverbound Serverbound {
            0x04 => ConfigurationKeepAliveServerbound_i64
        }
        clientbound Clientbound {
        }
    }
);
