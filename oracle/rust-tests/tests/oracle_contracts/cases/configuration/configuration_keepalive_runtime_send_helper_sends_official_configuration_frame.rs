#[test]
fn configuration_keepalive_runtime_send_helper_sends_official_configuration_frame() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/runtime/configuration_keepalive_runtime_send_helper.test-manifest.json");
    assert_eq!(
        manifest.case_id,
        "configuration_keepalive_runtime_send_helper"
    );
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/runtime/configuration_keepalive_runtime_send_helper.contract.json"
    );
    assert_eq!(manifest.answer_path, "oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "configuration_keepalive_runtime_send_helper_sends_official_configuration_frame"
    );
    assert_eq!(
        manifest.comparison_surface,
        "runtime_send_helper_frame"
    );
    assert_runner_scope("oracle/test-manifests/775/runtime/configuration_keepalive_runtime_send_helper.test-manifest.json", &manifest);

    let oracle = read_answer(
        &manifest.answer_path,
        "configuration_keepalive_framed_dispatch",
    );
    assert_eq!(oracle.case_id, "configuration_keepalive_framed_dispatch");
    assert_eq!(oracle.answer.decoded_id, Some(oracle.answer.input_id));
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:keep_alive")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let official_framed = decode_hex(
        oracle
            .answer
            .encoded_framed_hex
            .as_deref()
            .expect("framed dispatch answer missing encoded_framed_hex"),
        "configuration_keepalive_framed_dispatch.encoded_framed_hex",
    );
    let expected_network_frame = official_network_frame_from_framed_payload(&official_framed);

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind localhost probe server");
    let server_addr = listener.local_addr().expect("read localhost probe addr");
    let mut server = Some(thread::spawn(move || -> Result<Vec<u8>, String> {
        let (mut stream, _) = listener
            .accept()
            .map_err(|err| format!("accept runtime send probe client: {err}"))?;
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .map_err(|err| format!("set runtime send probe read timeout: {err}"))?;
        let packet_len = try_read_varint_from_reader(&mut stream)? as usize;
        let mut body = vec![0; packet_len];
        stream
            .read_exact(&mut body)
            .map_err(|err| format!("read runtime send probe packet body: {err}"))?;

        let mut observed = encode_varint(packet_len as i32);
        observed.extend_from_slice(&body);
        Ok(observed)
    }));

    let mut conn = Conn::new(&server_addr.to_string(), 775).expect("connect runtime send probe");
    conn.state = State::Configuration;
    let send_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::send_keep_alive(&mut conn, oracle.answer.input_id)
    }));
    match send_result {
        Ok(Ok(())) => {}
        Ok(Err(err)) => {
            drop(conn);
            if let Some(server) = server.take() {
                let _ = server.join();
            }
            panic!("packet::send_keep_alive returned an error in Configuration state: {err}");
        }
        Err(_) => {
            drop(conn);
            if let Some(server) = server.take() {
                let _ = server.join();
            }
            panic!(
                "packet::send_keep_alive panicked in Configuration state before sending the official Protocol 775 Configuration serverbound keep_alive frame"
            );
        }
    }

    let observed_network_frame = server
        .take()
        .expect("runtime send probe server was already joined")
        .join()
        .expect("runtime send probe server thread panicked")
        .expect("runtime send probe server did not observe a complete packet");

    assert_eq!(
        hex::encode(&observed_network_frame),
        hex::encode(&expected_network_frame),
        "packet::send_keep_alive in Configuration state must send the official Protocol 775 Configuration serverbound keep_alive framed packet with the outer network length prefix"
    );
}

