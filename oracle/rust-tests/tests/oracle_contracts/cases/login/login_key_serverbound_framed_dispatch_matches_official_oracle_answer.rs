#[test]
fn login_key_serverbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_key_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_key_serverbound_framed_dispatch_body)
        .expect("spawn login key oracle stack")
        .join()
        .expect("login key oracle thread panicked");
}

fn login_key_serverbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/login_key_serverbound_framed_dispatch.test-manifest.json");
    assert_eq!(manifest.case_id, "login_key_serverbound_framed_dispatch");
    assert_eq!(manifest.contract_path, "oracle/contracts/775/login_key_serverbound_framed_dispatch.contract.json");
    assert_eq!(manifest.answer_path, "oracle/answers/775/login_key_serverbound_framed_dispatch.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, "login_key_serverbound_framed_dispatch_matches_official_oracle_answer");
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/login_key_serverbound_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(oracle.answer.packet_type.as_deref(), Some("minecraft:key"));
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:key")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.login.ServerboundKeyPacket")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_keybytes_hex,
        oracle.answer.decoded_keybytes_hex
    );
    assert_eq!(
        oracle.answer.input_keybytes_length,
        oracle.answer.decoded_keybytes_length
    );
    assert_eq!(
        oracle.answer.input_encrypted_challenge_hex,
        oracle.answer.decoded_encrypted_challenge_hex
    );
    assert_eq!(
        oracle.answer.input_encrypted_challenge_length,
        oracle.answer.decoded_encrypted_challenge_length
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_serverbound_packet_table,
        "minecraft:key",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login key answer missing encoded_framed_hex");
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
            Direction::Serverbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Login serverbound key packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| panic!("Stevenarella errored while decoding login key packet: {err}"))
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login serverbound key packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::EncryptionResponse(packet) => {
            assert_eq!(
                hex::encode(packet.shared_secret.data),
                oracle.answer.decoded_keybytes_hex.as_deref().unwrap()
            );
            assert_eq!(
                hex::encode(packet.verify_token.data),
                oracle
                    .answer
                    .decoded_encrypted_challenge_hex
                    .as_deref()
                    .unwrap()
            );
        }
        other => panic!("expected Login serverbound key dispatch, got {other:?}"),
    }
    assert!(
        body_slice.is_empty(),
        "decoded login key packet did not consume the official body bytes"
    );
}

