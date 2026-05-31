#[test]
fn login_finished_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_finished_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_finished_clientbound_framed_dispatch_body)
        .expect("spawn login_finished_clientbound oracle stack")
        .join()
        .expect("login_finished_clientbound oracle thread panicked");
}

fn login_finished_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/login_finished_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(manifest.case_id, "login_finished_clientbound_framed_dispatch");
    assert_eq!(manifest.contract_path, "oracle/contracts/775/login_finished_clientbound_framed_dispatch.contract.json");
    assert_eq!(manifest.answer_path, "oracle/answers/775/login_finished_clientbound_framed_dispatch.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "login_finished_clientbound_framed_dispatch_matches_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/login_finished_clientbound_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:login_finished")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:login_finished")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket")
    );
    assert_eq!(
        oracle.answer.input_profile_id, oracle.answer.decoded_profile_id,
        "official decoded Login clientbound login_finished profile id differs from input"
    );
    assert_eq!(
        oracle.answer.input_profile_name, oracle.answer.decoded_profile_name,
        "official decoded Login clientbound login_finished profile name differs from input"
    );
    assert_eq!(
        oracle.answer.input_property_count, oracle.answer.decoded_property_count,
        "official decoded Login clientbound login_finished property count differs from input"
    );
    assert_eq!(oracle.answer.input_property_count, Some(0));
    assert_eq!(oracle.answer.input_is_terminal, Some(true));
    assert_eq!(
        oracle.answer.input_is_terminal, oracle.answer.decoded_is_terminal,
        "official decoded Login clientbound login_finished terminal flag differs from input"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_clientbound_packet_table,
        "minecraft:login_finished",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login_finished clientbound answer missing encoded_framed_hex");
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
            "Stevenarella panicked while dispatching official Login clientbound login_finished packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Login clientbound login_finished packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login clientbound login_finished packet id {}",
                framed_packet_id
            )
        });

    let expected_uuid: steven_protocol::protocol::UUID = oracle
        .answer
        .decoded_profile_id
        .as_deref()
        .expect("login_finished answer missing decoded_profile_id")
        .parse()
        .expect("official login_finished decoded_profile_id is not a Stevenarella UUID");
    let expected_name = oracle
        .answer
        .decoded_profile_name
        .as_deref()
        .expect("login_finished answer missing decoded_profile_name");

    match decoded {
        packet::Packet::LoginSuccess_UUID(packet) => {
            assert_eq!(
                packet.uuid, expected_uuid,
                "decoded Login clientbound login_finished uuid did not match the official GameProfile id"
            );
            assert_eq!(
                packet.username, expected_name,
                "decoded Login clientbound login_finished username did not match the official GameProfile name"
            );
        }
        other => {
            panic!("decoded packet did not preserve Login clientbound login_finished identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Login clientbound login_finished packet did not consume the official body bytes"
    );
}

