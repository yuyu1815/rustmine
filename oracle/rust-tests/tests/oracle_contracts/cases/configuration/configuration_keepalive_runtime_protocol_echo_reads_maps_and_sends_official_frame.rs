#[test]
fn configuration_keepalive_runtime_protocol_echo_reads_maps_and_sends_official_frame() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/runtime/configuration_keepalive_runtime_protocol_echo.test-manifest.json");
    assert_eq!(
        manifest.case_id,
        "configuration_keepalive_runtime_protocol_echo"
    );
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/runtime/configuration_keepalive_runtime_protocol_echo.contract.json"
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
        "configuration_keepalive_runtime_protocol_echo_reads_maps_and_sends_official_frame"
    );
    assert_eq!(
        manifest.comparison_surface,
        "runtime_protocol_echo_frame"
    );
    assert_runner_scope(
        "oracle/test-manifests/775/runtime/configuration_keepalive_runtime_protocol_echo.test-manifest.json",
        &manifest,
    );

    let inbound_oracle = read_answer(
        &manifest.answer_path,
        "configuration_keepalive_clientbound_framed_dispatch",
    );
    let outbound_answer_path = manifest
        .response_answer_path
        .as_deref()
        .expect("runtime protocol echo manifest missing response_answer_path");
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
        "runtime echo fixture requires the inbound and outbound official answers to use the same keep_alive id"
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
    let inbound_network_frame = official_network_frame_from_framed_payload(&inbound_framed);
    let expected_outbound_network_frame =
        official_network_frame_from_framed_payload(&outbound_framed);

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind localhost echo probe server");
    let server_addr = listener
        .local_addr()
        .expect("read localhost echo probe addr");
    let mut server = Some(thread::spawn(move || -> Result<Vec<u8>, String> {
        let (mut stream, _) = listener
            .accept()
            .map_err(|err| format!("accept runtime protocol echo probe client: {err}"))?;
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .map_err(|err| format!("set runtime protocol echo probe read timeout: {err}"))?;
        stream
            .set_write_timeout(Some(Duration::from_secs(2)))
            .map_err(|err| format!("set runtime protocol echo probe write timeout: {err}"))?;
        stream
            .write_all(&inbound_network_frame)
            .map_err(|err| format!("write official clientbound keep_alive frame: {err}"))?;

        read_network_frame_from_reader(&mut stream, "runtime protocol echo response")
    }));

    let mut conn = Conn::new(&server_addr.to_string(), 775).expect("connect protocol echo probe");
    conn.state = State::Configuration;
    let mapped = conn
        .read_packet()
        .expect("read official Configuration clientbound keep_alive frame")
        .map();
    let keep_alive_id = match mapped {
        steven_protocol::protocol::mapped_packet::MappedPacket::KeepAliveClientbound(
            keep_alive,
        ) => keep_alive.id,
        other => panic!("expected mapped clientbound keep_alive packet, got {other:?}"),
    };
    assert_eq!(keep_alive_id, inbound_oracle.answer.input_id);

    packet::send_keep_alive(&mut conn, keep_alive_id)
        .expect("send official Configuration serverbound keep_alive response");

    let observed_outbound_network_frame = server
        .take()
        .expect("runtime protocol echo probe server was already joined")
        .join()
        .expect("runtime protocol echo probe server thread panicked")
        .expect("runtime protocol echo probe server did not observe a complete response packet");

    assert_eq!(
        hex::encode(&observed_outbound_network_frame),
        hex::encode(&expected_outbound_network_frame),
        "runtime protocol echo path must map the official Configuration clientbound keep_alive id and send the official Configuration serverbound keep_alive framed packet with the outer network length prefix"
    );
}

