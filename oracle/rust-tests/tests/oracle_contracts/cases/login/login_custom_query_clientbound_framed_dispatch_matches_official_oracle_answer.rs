#[test]
fn login_custom_query_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_custom_query_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_custom_query_clientbound_framed_dispatch_body)
        .expect("spawn login_custom_query_clientbound oracle stack")
        .join()
        .expect("login_custom_query_clientbound oracle thread panicked");
}

fn login_custom_query_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/login_custom_query_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(manifest.case_id, "login_custom_query_clientbound_framed_dispatch");
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/login_custom_query_clientbound_framed_dispatch.contract.json"
    );
    assert_eq!(manifest.answer_path, "oracle/answers/775/login_custom_query_clientbound_framed_dispatch.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "login_custom_query_clientbound_framed_dispatch_matches_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/login_custom_query_clientbound_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:custom_query")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:custom_query")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.login.ClientboundCustomQueryPacket")
    );
    assert_eq!(
        oracle.answer.input_payload_class.as_deref(),
        Some("net.minecraft.network.protocol.login.custom.DiscardedQueryPayload")
    );
    assert_eq!(
        oracle.answer.decoded_payload_class.as_deref(),
        Some("net.minecraft.network.protocol.login.custom.DiscardedQueryPayload")
    );
    assert_eq!(
        oracle.answer.input_transaction_id, oracle.answer.decoded_transaction_id,
        "official decoded Login clientbound custom_query transaction id differs from input"
    );
    assert_eq!(
        oracle.answer.input_payload_id, oracle.answer.decoded_payload_id,
        "official decoded Login clientbound custom_query payload id differs from input"
    );
    assert_eq!(
        oracle.answer.input_payload_length, oracle.answer.decoded_payload_length,
        "official decoded Login clientbound custom_query payload length differs from input"
    );
    assert_eq!(
        oracle.answer.input_payload_length,
        Some(0),
        "fixture changed: expected empty DiscardedQueryPayload body"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_clientbound_packet_table,
        "minecraft:custom_query",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login_custom_query clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Login,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Login clientbound custom_query packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Login clientbound custom_query packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login clientbound custom_query packet id {}",
                framed_packet_id
            )
        });

    let expected_transaction_id = oracle
        .answer
        .decoded_transaction_id
        .expect("login_custom_query answer missing decoded_transaction_id");
    let expected_payload_id = oracle
        .answer
        .decoded_payload_id
        .as_deref()
        .expect("login_custom_query answer missing decoded_payload_id");
    let expected_payload_hex = oracle
        .answer
        .encoded_payload_body_hex
        .as_deref()
        .expect("login_custom_query answer missing encoded_payload_body_hex");
    let expected_payload = decode_hex(expected_payload_hex, "encoded_payload_body_hex");

    match decoded {
        packet::Packet::LoginPluginRequest(packet) => {
            assert_eq!(
                packet.message_id.0, expected_transaction_id,
                "decoded Login clientbound custom_query transaction id did not match official transactionId"
            );
            assert_eq!(
                packet.channel, expected_payload_id,
                "decoded Login clientbound custom_query channel did not match official payload id"
            );
            assert_eq!(
                packet.data, expected_payload,
                "decoded Login clientbound custom_query compatibility packet did not preserve payload bytes"
            );
        }
        other => {
            panic!("decoded packet did not preserve Login clientbound custom_query identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Login clientbound custom_query packet did not consume the official body bytes"
    );
}

