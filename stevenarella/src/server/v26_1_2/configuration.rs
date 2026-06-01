use crate::protocol;
use crate::protocol::mapped_packet;

use log::warn;

use super::super::Server;

pub(in crate::server) fn handle_plugin_message_clientbound(
    server: &Server,
    msg: &mapped_packet::play::clientbound::PluginMessageClientbound,
) -> bool {
    if msg.channel.as_str() != "FinishConfiguration" {
        return false;
    }

    transition_to_play(server);
    true
}

fn transition_to_play(server: &Server) {
    let mut conn = server.conn.write();
    let Some(conn) = conn.as_mut() else {
        return;
    };

    if conn.state != protocol::State::Configuration {
        return;
    }

    if conn
        .write_packet(
            protocol::packet::configuration::serverbound::ConfigurationFinishConfigurationServerbound {
                empty: (),
            },
        )
        .is_err()
    {
        warn!("Failed to send ConfigurationFinishConfiguration to server");
        return;
    }

    conn.state = protocol::State::Play;
}
