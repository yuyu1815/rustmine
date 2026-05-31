#[test]
fn login_compression_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_compression_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_compression_clientbound_framed_dispatch_body)
        .expect("spawn login_compression_clientbound oracle stack")
        .join()
        .expect("login_compression_clientbound oracle thread panicked");
}

fn login_compression_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/login_compression_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(manifest.case_id, "login_compression_clientbound_framed_dispatch");
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/login_compression_clientbound_framed_dispatch.contract.json"
    );
    assert_eq!(manifest.answer_path, "oracle/answers/775/login_compression_clientbound_framed_dispatch.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "login_compression_clientbound_framed_dispatch_matches_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/login_compression_clientbound_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:login_compression")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:login_compression")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.login.ClientboundLoginCompressionPacket")
    );
    assert_eq!(
        oracle.answer.input_compression_threshold, oracle.answer.decoded_compression_threshold,
        "official decoded Login clientbound login_compression threshold differs from input"
    );
    assert_eq!(
        oracle.answer.input_compression_threshold,
        Some(0),
        "fixture changed: expected smallest accepted compression threshold"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_clientbound_packet_table,
        "minecraft:login_compression",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login_compression clientbound answer missing encoded_framed_hex");
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
            "Stevenarella panicked while dispatching official Login clientbound login_compression packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Login clientbound login_compression packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login clientbound login_compression packet id {}",
                framed_packet_id
            )
        });

    let expected_threshold = oracle
        .answer
        .decoded_compression_threshold
        .expect("login_compression answer missing decoded_compression_threshold");

    match decoded {
        packet::Packet::SetInitialCompression(packet) => {
            assert_eq!(
                packet.threshold.0, expected_threshold,
                "decoded Login clientbound login_compression threshold did not match official compressionThreshold"
            );
        }
        other => {
            panic!("decoded packet did not preserve Login clientbound login_compression identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Login clientbound login_compression packet did not consume the official body bytes"
    );
}

