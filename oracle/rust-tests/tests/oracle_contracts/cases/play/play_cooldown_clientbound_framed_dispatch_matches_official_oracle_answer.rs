#[test]
fn play_cooldown_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_cooldown_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_cooldown_clientbound_framed_dispatch_body)
        .expect("spawn play_cooldown_clientbound oracle stack")
        .join()
        .expect("play_cooldown_clientbound oracle thread panicked");
}

fn play_cooldown_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/play_cooldown_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(manifest.case_id, "play_cooldown_clientbound_framed_dispatch");
    assert_eq!(manifest.contract_path, "oracle/contracts/775/play_cooldown_clientbound_framed_dispatch.contract.json");
    assert_eq!(manifest.answer_path, "oracle/answers/775/play_cooldown_clientbound_framed_dispatch.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, "play_cooldown_clientbound_framed_dispatch_matches_official_oracle_answer");
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/play_cooldown_clientbound_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:cooldown")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:cooldown")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundCooldownPacket")
    );
    assert_eq!(
        oracle.answer.input_cooldown_group,
        oracle.answer.decoded_cooldown_group
    );
    assert_eq!(
        oracle.answer.stream_decoded_cooldown_group,
        oracle.answer.decoded_cooldown_group
    );
    assert_eq!(oracle.answer.input_duration, oracle.answer.decoded_duration);
    assert_eq!(
        oracle.answer.stream_decoded_duration,
        oracle.answer.decoded_duration
    );
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:cooldown",
    );
    let framed = decode_hex(
        oracle
            .answer
            .encoded_framed_hex
            .as_deref()
            .expect("play_cooldown answer missing encoded_framed_hex"),
        "encoded_framed_hex",
    );
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_cooldown answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(body, fixture_body);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound cooldown")
    .expect("dispatch Play clientbound cooldown");

    match decoded {
        packet::Packet::PlayCooldownClientbound(cooldown) => {
            assert_eq!(
                cooldown.cooldown_group,
                oracle.answer.decoded_cooldown_group.unwrap_or_default()
            );
            assert_eq!(
                cooldown.duration.0,
                oracle
                    .answer
                    .decoded_duration
                    .expect("missing decoded_duration")
            );
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound cooldown identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}

