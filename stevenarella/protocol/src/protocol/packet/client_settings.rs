use crate::protocol::{packet, Conn, Error, VarInt};
use crate::shared::Version;

use super::Hand;

pub fn send_client_settings(
    conn: &mut Conn,
    locale: String,
    view_distance: u8,
    chat_mode: u8,
    chat_colors: bool,
    displayed_skin_parts: u8,
    main_hand: Hand,
) -> Result<(), Error> {
    let version = conn.get_version();
    if version < Version::V1_9 {
        // TODO: Do this for protocol version 48
        // 1 snapshot after 1.8
        conn.write_packet(packet::play::serverbound::ClientSettings_u8_Handsfree {
            locale,
            view_distance,
            chat_mode,
            chat_colors,
            displayed_skin_parts,
        })
    } else {
        conn.write_packet(packet::play::serverbound::ClientSettings {
            locale,
            view_distance,
            chat_mode: VarInt(chat_mode as i32),
            chat_colors,
            displayed_skin_parts,
            main_hand: VarInt(main_hand.ordinal()),
        })
    }
}
