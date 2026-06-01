mod internal_protocol;
mod packets;

pub use packets::translate_internal_packet_id;

pub(crate) fn read_internal_protocol_packet_by_id<R: std::io::Read>(
    state: crate::protocol::State,
    dir: crate::protocol::Direction,
    id: i32,
    buf: &mut R,
) -> Result<Option<crate::protocol::packet::Packet>, crate::protocol::Error> {
    match (state, dir) {
        (crate::protocol::State::Login, _) => {
            internal_protocol::login::read_login_packet_by_id(dir, id, buf)
        }
        (crate::protocol::State::Configuration, _) => {
            internal_protocol::configuration::read_configuration_packet_by_id(dir, id, buf)
        }
        (crate::protocol::State::Play, crate::protocol::Direction::Clientbound) => {
            internal_protocol::play_clientbound::read_play_clientbound_packet_by_id(id, buf)
        }
        _ => Ok(None),
    }
}
