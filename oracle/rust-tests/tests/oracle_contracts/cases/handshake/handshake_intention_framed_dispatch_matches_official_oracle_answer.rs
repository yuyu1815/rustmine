#[test]
fn handshake_intention_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("handshake_intention_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(handshake_intention_framed_dispatch_body)
        .expect("spawn handshake intention oracle stack")
        .join()
        .expect("handshake intention oracle thread panicked");
}

fn handshake_intention_framed_dispatch_body() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/handshake_intention_framed_dispatch.test-manifest.json");
    assert_eq!(manifest.case_id, "handshake_intention_framed_dispatch");
    assert_eq!(manifest.contract_path, "oracle/contracts/775/handshake_intention_framed_dispatch.contract.json");
    assert_eq!(manifest.answer_path, "oracle/answers/775/handshake_intention_framed_dispatch.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, "handshake_intention_framed_dispatch_matches_official_oracle_answer");
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/handshake_intention_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:intention")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.handshake.ClientIntentionPacket")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(oracle.answer.input_protocol_version, Some(775));
    assert_eq!(oracle.answer.decoded_protocol_version, Some(775));
    assert_eq!(oracle.answer.input_host.as_deref(), Some("localhost"));
    assert_eq!(oracle.answer.decoded_host.as_deref(), Some("localhost"));
    assert_eq!(oracle.answer.input_port, Some(25565));
    assert_eq!(oracle.answer.decoded_port, Some(25565));
    assert_eq!(oracle.answer.input_intent.as_deref(), Some("LOGIN"));
    assert_eq!(oracle.answer.decoded_intent.as_deref(), Some("LOGIN"));
    assert_eq!(oracle.answer.input_intent_id, Some(2));
    assert_eq!(oracle.answer.decoded_intent_id, Some(2));
    assert_eq!(oracle.answer.input_is_terminal, Some(true));
    assert_eq!(oracle.answer.decoded_is_terminal, Some(true));

    let expected_packet_id = packet_id_for(
        &oracle.answer.handshaking_serverbound_packet_table,
        "minecraft:intention",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("framed dispatch answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Handshaking,
        Direction::Serverbound,
        framed_packet_id,
        &mut body_slice,
    )
    .unwrap()
    .expect("expected Handshaking serverbound intention to dispatch");

    match decoded {
        packet::Packet::Handshake(packet) => {
            assert_eq!(
                packet.protocol_version.0,
                oracle.answer.decoded_protocol_version.unwrap()
            );
            assert_eq!(packet.host, oracle.answer.decoded_host.as_deref().unwrap());
            assert_eq!(i32::from(packet.port), oracle.answer.decoded_port.unwrap());
            assert_eq!(packet.next.0, oracle.answer.decoded_intent_id.unwrap());
        }
        other => panic!("expected intention dispatch, got {other:?}"),
    }
    assert!(
        body_slice.is_empty(),
        "decoded packet did not consume the official body bytes"
    );
}

