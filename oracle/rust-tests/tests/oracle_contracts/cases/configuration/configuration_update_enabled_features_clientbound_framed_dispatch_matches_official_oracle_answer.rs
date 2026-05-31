#[test]
fn configuration_update_enabled_features_clientbound_framed_dispatch_matches_official_oracle_answer(
) {
    let manifest: TestManifest =
        read_json("oracle/test-manifests/775/configuration_update_enabled_features_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(
        manifest.case_id,
        "configuration_update_enabled_features_clientbound_framed_dispatch"
    );
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/configuration_update_enabled_features_clientbound_framed_dispatch.contract.json"
    );
    assert_eq!(
        manifest.answer_path,
        "oracle/answers/775/configuration_update_enabled_features_clientbound_framed_dispatch.answer.jsonl"
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "configuration_update_enabled_features_clientbound_framed_dispatch_matches_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope(
        "oracle/test-manifests/775/configuration_update_enabled_features_clientbound_framed_dispatch.test-manifest.json",
        &manifest,
    );

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:update_enabled_features")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:update_enabled_features")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.configuration.ClientboundUpdateEnabledFeaturesPacket")
    );
    assert_eq!(
        oracle.answer.input_fixture.as_deref(),
        Some("Set.of() features")
    );
    assert_eq!(
        oracle.answer.input_feature_count, oracle.answer.decoded_feature_count,
        "official decoded update_enabled_features count differs from the official input count"
    );
    assert_eq!(
        oracle.answer.input_features, oracle.answer.decoded_features,
        "official decoded update_enabled_features set differs from the official input set"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:update_enabled_features",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("update_enabled_features answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body,
        encode_varint(0),
        "official empty update_enabled_features fixture should encode a zero-length feature collection"
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
            "Stevenarella panicked while dispatching official Configuration clientbound update_enabled_features packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding clientbound update_enabled_features packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound update_enabled_features packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "UpdateEnabledFeatures",
                "decoded packet did not preserve update_enabled_features compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded update_enabled_features compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve clientbound update_enabled_features identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound update_enabled_features packet did not consume the official body bytes"
    );
}

