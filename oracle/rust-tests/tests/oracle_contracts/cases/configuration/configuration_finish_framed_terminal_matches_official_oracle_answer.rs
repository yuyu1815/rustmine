#[test]
fn configuration_finish_framed_terminal_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/configuration_finish_framed_terminal.test-manifest.json");
    assert_eq!(manifest.case_id, "configuration_finish_framed_terminal");
    assert_eq!(manifest.contract_path, "oracle/contracts/775/configuration_finish_framed_terminal.contract.json");
    assert_eq!(manifest.answer_path, "oracle/answers/775/configuration_finish_framed_terminal.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, "configuration_finish_framed_terminal_matches_official_oracle_answer");
    assert_eq!(
        manifest.comparison_surface,
        "decoded_fields"
    );
    assert_runner_scope("oracle/test-manifests/775/configuration_finish_framed_terminal.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    let packet_type = oracle
        .answer
        .packet_type
        .as_deref()
        .expect("finish_configuration answer missing packet_type");

    let serverbound = oracle
        .answer
        .serverbound
        .as_ref()
        .expect("finish_configuration answer missing serverbound direction");
    assert_eq!(serverbound.packet_type, packet_type);
    assert_finish_direction_matches_official_frame(serverbound, Direction::Serverbound);

    let clientbound = oracle
        .answer
        .clientbound
        .as_ref()
        .expect("finish_configuration answer missing clientbound direction");
    assert_eq!(clientbound.packet_type, packet_type);
    assert_finish_direction_matches_official_frame(clientbound, Direction::Clientbound);
}

