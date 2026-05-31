#[test]
fn configuration_registry_data_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/configuration_registry_data_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(
        manifest.case_id,
        "configuration_registry_data_clientbound_framed_dispatch"
    );
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/configuration_registry_data_clientbound_framed_dispatch.contract.json"
    );
    assert_eq!(
        manifest.answer_path,
        "oracle/answers/775/configuration_registry_data_clientbound_framed_dispatch.answer.jsonl"
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "configuration_registry_data_clientbound_framed_dispatch_matches_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/configuration_registry_data_clientbound_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:registry_data")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:registry_data")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.configuration.ClientboundRegistryDataPacket")
    );
    assert_eq!(
        oracle.answer.registry_fixture.as_deref(),
        Some("Registries.DIMENSION_TYPE with List.of() entries")
    );
    assert_eq!(
        oracle.answer.input_registry_key.as_deref(),
        Some("minecraft:dimension_type")
    );
    assert_eq!(
        oracle.answer.input_registry_key, oracle.answer.decoded_registry_key,
        "official decoded registry key differs from the official input registry key"
    );
    assert_eq!(
        oracle.answer.input_entry_count,
        Some(0),
        "registry_data fixture must not invent registry entries"
    );
    assert_eq!(
        oracle.answer.input_entry_count, oracle.answer.decoded_entry_count,
        "official decoded registry entry count differs from the official input entry count"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:registry_data",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("clientbound registry_data answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert!(
        !body.is_empty(),
        "official registry_data body should include registry key and empty entry-list length"
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
            "Stevenarella panicked while dispatching official Configuration clientbound registry_data packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound registry_data packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound registry_data packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "RegistryData",
                "decoded packet did not preserve registry_data compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded registry_data compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!("decoded packet did not preserve clientbound registry_data identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound registry_data packet did not consume the official body bytes"
    );
}

