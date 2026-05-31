#[test]
fn configuration_keepalive_matches_official_oracle_answer() {
    // Case-local typed adapter. Future cases should be dispatched from manifest
    // topology instead of copying this shape as the general oracle runner model.
    let manifest: TestManifest = read_json("oracle/test-manifests/775/configuration_keepalive_codec.test-manifest.json");
    assert_eq!(manifest.case_id, "configuration_keepalive_codec");
    assert_eq!(manifest.contract_path, "oracle/contracts/775/configuration_keepalive_codec.contract.json");
    assert_eq!(manifest.answer_path, "oracle/answers/775/configuration_keepalive_codec.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, "configuration_keepalive_matches_official_oracle_answer");
    assert_eq!(
        manifest.comparison_surface,
        "codec_body_only"
    );
    assert_runner_scope("oracle/test-manifests/775/configuration_keepalive_codec.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_serverbound_packet_table,
        "minecraft:keep_alive",
    );
    let packet = packet::configuration::serverbound::ConfigurationKeepAliveServerbound_i64 {
        id: oracle.answer.input_id,
    };

    assert_eq!(packet.packet_id(775), expected_packet_id);

    let mut body = Vec::new();
    packet.write(&mut body).unwrap();

    assert_eq!(hex::encode(body), oracle.answer.encoded_body_hex);
}

