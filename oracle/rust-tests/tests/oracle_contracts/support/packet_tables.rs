fn packet_id_for(table: &[PacketTableRow], packet_type: &str) -> i32 {
    table
        .iter()
        .find(|row| row.packet_type == packet_type)
        .unwrap_or_else(|| panic!("missing packet id for {packet_type}"))
        .packet_id
}

fn read_play_clientbound_oracle(
    manifest_path: &str,
    case_id: &str,
    contract_path: &str,
    answer_path: &str,
    test_name: &str,
    comparison_surface: &str,
    packet_type: &str,
) -> (OracleAnswer, i32, Vec<u8>, Vec<u8>) {
    let manifest: TestManifest = read_json(manifest_path);
    assert_eq!(manifest.case_id, case_id);
    assert_eq!(manifest.contract_path, contract_path);
    assert_eq!(manifest.answer_path, answer_path);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, test_name);
    assert_eq!(manifest.comparison_surface, comparison_surface);
    assert_runner_scope(manifest_path, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(oracle.answer.packet_type.as_deref(), Some(packet_type));
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some(packet_type)
    );
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id =
        packet_id_for(&oracle.answer.play_clientbound_packet_table, packet_type);
    let framed = decode_hex(
        oracle
            .answer
            .encoded_framed_hex
            .as_deref()
            .expect("Play clientbound answer missing encoded_framed_hex"),
        "encoded_framed_hex",
    );
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("Play clientbound answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(body, fixture_body);

    (oracle, framed_packet_id, body, fixture_body)
}
