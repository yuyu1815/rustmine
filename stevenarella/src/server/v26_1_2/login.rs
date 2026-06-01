use crate::protocol;

pub(in crate::server) fn transition_after_login(
    conn: &mut protocol::Conn,
) -> Result<(), protocol::Error> {
    conn.write_packet(protocol::packet::login::serverbound::LoginAcknowledged { empty: () })?;
    conn.state = protocol::State::Configuration;
    Ok(())
}
