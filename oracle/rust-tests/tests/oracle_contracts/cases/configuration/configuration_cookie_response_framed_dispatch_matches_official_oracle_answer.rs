#[test]
fn configuration_cookie_response_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/configuration_cookie_response_framed_dispatch.test-manifest.json");
    assert_eq!(manifest.case_id, "configuration_cookie_response_framed_dispatch");
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/configuration_cookie_response_framed_dispatch.contract.json"
    );
    assert_eq!(manifest.answer_path, "oracle/answers/775/configuration_cookie_response_framed_dispatch.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "configuration_cookie_response_framed_dispatch_matches_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/configuration_cookie_response_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:cookie_response")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:cookie_response")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.cookie.ServerboundCookieResponsePacket")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_key, oracle.answer.decoded_key,
        "official decoded cookie_response key differs from the official input key"
    );
    assert_eq!(
        oracle.answer.input_payload_hex, oracle.answer.decoded_payload_hex,
        "official decoded cookie_response payload differs from the official input payload"
    );
    assert_eq!(
        oracle.answer.input_payload_length, oracle.answer.decoded_payload_length,
        "official decoded cookie_response payload length differs from the official input payload"
    );
    assert_eq!(
        oracle.answer.decoded_payload_equals_input,
        Some(true),
        "official decoded cookie_response payload was not byte-equal to the input payload"
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_serverbound_packet_table,
        "minecraft:cookie_response",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("cookie_response answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Configuration,
            Direction::Serverbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Configuration serverbound cookie_response packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding cookie_response packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration serverbound cookie_response packet id {}",
                framed_packet_id
            )
        });
    let expected_payload_hex = oracle
        .answer
        .decoded_payload_hex
        .as_deref()
        .expect("cookie_response answer missing decoded_payload_hex");
    let expected_payload = decode_hex(expected_payload_hex, "decoded_payload_hex");
    match decoded {
        packet::Packet::PluginMessageServerbound(packet) => {
            assert_eq!(
                packet.channel, "CookieResponse",
                "decoded packet did not preserve cookie_response compatibility channel"
            );
            assert_eq!(
                packet.data, expected_payload,
                "decoded cookie_response compatibility packet did not preserve payload bytes"
            );
        }
        other => panic!("decoded packet did not preserve cookie_response identity: {other:?}"),
    }
    assert!(
        body_slice.is_empty(),
        "decoded cookie_response packet did not consume the official body bytes"
    );
}

