#[test]
fn configuration_select_known_packs_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/configuration_select_known_packs_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(
        manifest.case_id,
        "configuration_select_known_packs_clientbound_framed_dispatch"
    );
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/configuration_select_known_packs_clientbound_framed_dispatch.contract.json"
    );
    assert_eq!(
        manifest.answer_path,
        "oracle/answers/775/configuration_select_known_packs_clientbound_framed_dispatch.answer.jsonl"
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "configuration_select_known_packs_clientbound_framed_dispatch_matches_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope(
        "oracle/test-manifests/775/configuration_select_known_packs_clientbound_framed_dispatch.test-manifest.json",
        &manifest,
    );

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:select_known_packs")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:select_known_packs")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.configuration.ClientboundSelectKnownPacks")
    );
    assert_eq!(
        oracle.answer.input_fixture.as_deref(),
        Some("List.of() known_packs")
    );
    assert_eq!(
        oracle.answer.input_known_pack_count,
        Some(0),
        "clientbound select_known_packs fixture must not invent known-pack entries"
    );
    assert_eq!(
        oracle.answer.input_known_pack_count, oracle.answer.decoded_known_pack_count,
        "official decoded clientbound select_known_packs list length differs from the official input length"
    );
    assert_eq!(
        oracle.answer.input_known_packs, oracle.answer.decoded_known_packs,
        "official decoded clientbound select_known_packs list differs from the official input list"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:select_known_packs",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("clientbound select_known_packs answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body,
        encode_varint(0),
        "official empty clientbound select_known_packs fixture should encode a zero-length known-pack list"
    );

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
            "Stevenarella panicked while dispatching official Configuration clientbound select_known_packs packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding clientbound select_known_packs packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound select_known_packs packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "SelectKnownPacks",
                "decoded packet did not preserve clientbound select_known_packs compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded clientbound select_known_packs compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve clientbound select_known_packs identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound select_known_packs packet did not consume the official body bytes"
    );
}

