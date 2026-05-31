#[test]
fn configuration_keepalive_runtime_spawn_reader_reaction_echoes_official_frame() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/runtime/configuration_keepalive_runtime_spawn_reader_reaction.test-manifest.json");
    assert_eq!(
        manifest.case_id,
        "configuration_keepalive_runtime_spawn_reader_reaction"
    );
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/runtime/configuration_keepalive_runtime_spawn_reader_reaction.contract.json"
    );
    assert_eq!(
        manifest.answer_path,
        "oracle/answers/775/configuration_keepalive_clientbound_framed_dispatch.answer.jsonl"
    );
    assert_eq!(
        manifest.response_answer_path.as_deref(),
        Some("oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl")
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "configuration_keepalive_runtime_spawn_reader_reaction_echoes_official_frame"
    );
    assert_eq!(
        manifest.comparison_surface,
        "runtime_spawn_reader_reaction_frame"
    );
    assert_runner_scope(
        "oracle/test-manifests/775/runtime/configuration_keepalive_runtime_spawn_reader_reaction.test-manifest.json",
        &manifest,
    );

    let inbound_oracle = read_answer(
        &manifest.answer_path,
        "configuration_keepalive_clientbound_framed_dispatch",
    );
    let outbound_answer_path = manifest
        .response_answer_path
        .as_deref()
        .expect("runtime spawn_reader manifest missing response_answer_path");
    let outbound_oracle = read_answer(outbound_answer_path, "configuration_keepalive_framed_dispatch");

    assert_eq!(
        inbound_oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:keep_alive")
    );
    assert_eq!(
        inbound_oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundKeepAlivePacket")
    );
    assert_eq!(
        inbound_oracle.answer.remaining_after_official_decode,
        Some(0)
    );
    assert_eq!(
        outbound_oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:keep_alive")
    );
    assert_eq!(
        outbound_oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ServerboundKeepAlivePacket")
    );
    assert_eq!(
        outbound_oracle.answer.remaining_after_official_decode,
        Some(0)
    );
    assert_eq!(
        inbound_oracle.answer.input_id, outbound_oracle.answer.input_id,
        "spawn_reader fixture requires the inbound and outbound official answers to use the same keep_alive id"
    );

    let inbound_framed = decode_hex(
        inbound_oracle
            .answer
            .encoded_framed_hex
            .as_deref()
            .expect("clientbound framed dispatch answer missing encoded_framed_hex"),
        "configuration_keepalive_clientbound_framed_dispatch.encoded_framed_hex",
    );
    let outbound_framed = decode_hex(
        outbound_oracle
            .answer
            .encoded_framed_hex
            .as_deref()
            .expect("serverbound framed dispatch answer missing encoded_framed_hex"),
        "configuration_keepalive_framed_dispatch.encoded_framed_hex",
    );
    let _inbound_network_frame = official_network_frame_from_framed_payload(&inbound_framed);
    let _expected_outbound_network_frame =
        official_network_frame_from_framed_payload(&outbound_framed);

    let server_source_path = project_root().join("stevenarella/src/server/mod.rs");
    let server_source = fs::read_to_string(&server_source_path).unwrap_or_else(|err| {
        panic!(
            "failed to read spawn_reader owner source {}: {err}",
            server_source_path.display()
        )
    });
    assert!(
        server_source.contains("fn spawn_reader(mut read: protocol::Conn"),
        "expected current spawn_reader owner symbol in {}",
        server_source_path.display()
    );
    assert!(
        server_source.contains("MappedPacket::KeepAliveClientbound(keep_alive)"),
        "expected current spawn_reader keep_alive reaction branch in {}",
        server_source_path.display()
    );
    assert!(
        server_source.contains("pub fn handle_next_reader_packet_for_oracle"),
        "expected narrow oracle reader-loop helper in {}",
        server_source_path.display()
    );
    assert!(
        server_source.contains("conn.state = protocol::State::Play;"),
        "expected current public Server::connect state transition evidence in {}",
        server_source_path.display()
    );

    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let output = Command::new(cargo)
        .current_dir(project_root())
        .args([
            "test",
            "--manifest-path",
            "stevenarella/Cargo.toml",
            "server::tests::configuration_keepalive_runtime_spawn_reader_reaction_echoes_official_frame",
            "--",
            "--exact",
        ])
        .output()
        .expect("run stevenarella spawn_reader oracle probe");

    assert!(
        output.status.success(),
        "stevenarella spawn_reader oracle probe failed\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

