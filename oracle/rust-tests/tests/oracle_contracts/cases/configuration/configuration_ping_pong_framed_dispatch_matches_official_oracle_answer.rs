#[test]
fn configuration_ping_pong_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/configuration_ping_pong_framed_dispatch.test-manifest.json");
    assert_eq!(manifest.case_id, "configuration_ping_pong_framed_dispatch");
    assert_eq!(manifest.contract_path, "oracle/contracts/775/configuration_ping_pong_framed_dispatch.contract.json");
    assert_eq!(manifest.answer_path, "oracle/answers/775/configuration_ping_pong_framed_dispatch.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, "configuration_ping_pong_framed_dispatch_matches_official_oracle_answer");
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/configuration_ping_pong_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);

    let clientbound_ping = oracle
        .answer
        .clientbound_ping
        .as_ref()
        .expect("ping/pong answer missing clientbound_ping direction");
    assert_ping_pong_direction_matches_official_frame(clientbound_ping, Direction::Clientbound);

    let serverbound_pong = oracle
        .answer
        .serverbound_pong
        .as_ref()
        .expect("ping/pong answer missing serverbound_pong direction");
    assert_ping_pong_direction_matches_official_frame(serverbound_pong, Direction::Serverbound);
}

