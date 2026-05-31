#[test]
fn login_hello_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_hello_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_hello_clientbound_framed_dispatch_body)
        .expect("spawn login_hello_clientbound oracle stack")
        .join()
        .expect("login_hello_clientbound oracle thread panicked");
}

fn login_hello_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/login_hello_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(manifest.case_id, "login_hello_clientbound_framed_dispatch");
    assert_eq!(manifest.contract_path, "oracle/contracts/775/login_hello_clientbound_framed_dispatch.contract.json");
    assert_eq!(manifest.answer_path, "oracle/answers/775/login_hello_clientbound_framed_dispatch.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, "login_hello_clientbound_framed_dispatch_matches_official_oracle_answer");
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/login_hello_clientbound_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:hello")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:hello")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.login.ClientboundHelloPacket")
    );
    assert_eq!(
        oracle.answer.input_server_id, oracle.answer.decoded_server_id,
        "official decoded Login clientbound hello serverId differs from input"
    );
    assert_eq!(
        oracle.answer.input_public_key_hex, oracle.answer.decoded_public_key_hex,
        "official decoded Login clientbound hello publicKey differs from input"
    );
    assert_eq!(
        oracle.answer.input_public_key_length, oracle.answer.decoded_public_key_length,
        "official decoded Login clientbound hello publicKey length differs from input"
    );
    assert_eq!(
        oracle.answer.input_challenge_hex, oracle.answer.decoded_challenge_hex,
        "official decoded Login clientbound hello challenge differs from input"
    );
    assert_eq!(
        oracle.answer.input_challenge_length, oracle.answer.decoded_challenge_length,
        "official decoded Login clientbound hello challenge length differs from input"
    );
    assert_eq!(
        oracle.answer.input_should_authenticate, oracle.answer.decoded_should_authenticate,
        "official decoded Login clientbound hello shouldAuthenticate differs from input"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_clientbound_packet_table,
        "minecraft:hello",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login_hello clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Login,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Login clientbound hello packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Login clientbound hello packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login clientbound hello packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::EncryptionRequest_ShouldAuthenticate(packet) => {
            assert_eq!(
                packet.server_id,
                oracle.answer.decoded_server_id.clone().unwrap_or_default(),
                "decoded Login clientbound hello server_id did not match the official serverId"
            );
            assert_eq!(
                hex::encode(packet.public_key.data),
                oracle
                    .answer
                    .decoded_public_key_hex
                    .clone()
                    .unwrap_or_default(),
                "decoded Login clientbound hello public_key did not match the official publicKey"
            );
            assert_eq!(
                hex::encode(packet.verify_token.data),
                oracle
                    .answer
                    .decoded_challenge_hex
                    .clone()
                    .unwrap_or_default(),
                "decoded Login clientbound hello verify_token did not match the official challenge"
            );
            assert_eq!(
                packet.should_authenticate,
                oracle.answer.decoded_should_authenticate.unwrap_or(false),
                "decoded Login clientbound hello should_authenticate did not match the official shouldAuthenticate"
            );
        }
        other => {
            panic!("decoded packet did not preserve Login clientbound hello identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Login clientbound hello packet did not consume the official body bytes"
    );
}

