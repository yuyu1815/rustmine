#[test]
fn configuration_keepalive_clientbound_framed_dispatch_decodes_official_oracle_answer() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/configuration_keepalive_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(
        manifest.case_id,
        "configuration_keepalive_clientbound_framed_dispatch"
    );
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/configuration_keepalive_clientbound_framed_dispatch.contract.json"
    );
    assert_eq!(
        manifest.answer_path,
        "oracle/answers/775/configuration_keepalive_clientbound_framed_dispatch.answer.jsonl"
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "configuration_keepalive_clientbound_framed_dispatch_decodes_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope(
        "oracle/test-manifests/775/configuration_keepalive_clientbound_framed_dispatch.test-manifest.json",
        &manifest,
    );

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(oracle.answer.decoded_id, Some(oracle.answer.input_id));
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:keep_alive")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundKeepAlivePacket")
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:keep_alive",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("clientbound framed dispatch answer missing encoded_framed_hex");
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
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Configuration clientbound keep_alive packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| panic!("Stevenarella errored while decoding clientbound keep_alive: {err}"))
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound keep_alive packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::KeepAliveClientbound_i64(packet) => {
            assert_eq!(packet.id, oracle.answer.input_id);
        }
        other => panic!("expected clientbound keep_alive dispatch, got {other:?}"),
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound keep_alive did not consume the official body bytes"
    );
}

