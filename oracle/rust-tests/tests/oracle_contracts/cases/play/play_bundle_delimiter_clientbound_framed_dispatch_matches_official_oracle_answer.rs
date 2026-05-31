#[test]
fn play_bundle_delimiter_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_bundle_delimiter_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_bundle_delimiter_clientbound_framed_dispatch_body)
        .expect("spawn play_bundle_delimiter_clientbound oracle stack")
        .join()
        .expect("play_bundle_delimiter_clientbound oracle thread panicked");
}

fn play_bundle_delimiter_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/play_bundle_delimiter_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(manifest.case_id, "play_bundle_delimiter_clientbound_framed_dispatch");
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/play_bundle_delimiter_clientbound_framed_dispatch.contract.json"
    );
    assert_eq!(
        manifest.answer_path,
        "oracle/answers/775/play_bundle_delimiter_clientbound_framed_dispatch.answer.jsonl"
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "play_bundle_delimiter_clientbound_framed_dispatch_matches_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/play_bundle_delimiter_clientbound_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:bundle_delimiter")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:bundle_delimiter")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket")
    );
    assert_eq!(oracle.answer.encoded_body_hex, "");
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:bundle_delimiter",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_bundle_delimiter clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound bundle_delimiter packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Play clientbound bundle_delimiter packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound bundle_delimiter packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::BundleDelimiterClientbound(packet) => {
            assert_eq!(
                packet.empty,
                (),
                "decoded Play clientbound bundle_delimiter must preserve empty body"
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound bundle_delimiter identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound bundle_delimiter packet did not consume the official body bytes"
    );
}

