#[derive(Debug, Deserialize)]
struct PacketTableRow {
    packet_id: i32,
    packet_type: String,
}

#[derive(Debug, Deserialize)]
struct ResourcePackActionRow {
    name: String,
    is_terminal: bool,
}

#[derive(Debug, Deserialize)]
struct FinishDirectionAnswer {
    flow: String,
    packet_type: String,
    decoded_packet_type: String,
    decoded_packet_class: String,
    instance_is_terminal: bool,
    decoded_is_terminal: bool,
    encoded_framed_hex: String,
    encoded_body_hex: String,
    remaining_after_official_decode: i32,
    configuration_packet_table: Vec<PacketTableRow>,
}

#[derive(Debug, Deserialize)]
struct FramedDirectionAnswer {
    flow: String,
    packet_type: String,
    decoded_packet_type: String,
    decoded_packet_class: String,
    input_id: i32,
    encoded_framed_hex: String,
    encoded_body_hex: String,
    decoded_id: i32,
    remaining_after_official_decode: i32,
    configuration_packet_table: Vec<PacketTableRow>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ClientInformationRecord {
    language: String,
    view_distance: i32,
    chat_visibility: String,
    chat_colors: bool,
    model_customisation: i32,
    main_hand: String,
    text_filtering_enabled: bool,
    allows_listing: bool,
    particle_status: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct KnownPackRecord {
    namespace: String,
    id: String,
    version: String,
    is_vanilla: bool,
}
