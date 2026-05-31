#[test]
fn play_set_score_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_set_score_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_set_score_clientbound_framed_dispatch_body)
        .expect("spawn play_set_score_clientbound oracle stack")
        .join()
        .expect("play_set_score_clientbound oracle thread panicked");
}

fn play_set_score_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_score_clientbound_framed_dispatch.test-manifest.json",
        "play_set_score_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_score_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_score_clientbound_framed_dispatch.answer.jsonl",
        "play_set_score_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_score",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetScorePacket")
    );
    assert_eq!(oracle.answer.input_owner.as_deref(), Some("rustmine_owner"));
    assert_eq!(
        oracle.answer.input_owner,
        oracle.answer.stream_decoded_owner
    );
    assert_eq!(oracle.answer.input_owner, oracle.answer.decoded_owner);
    assert_eq!(
        oracle.answer.input_objective_name.as_deref(),
        Some("rustmine_objective")
    );
    assert_eq!(
        oracle.answer.input_objective_name,
        oracle.answer.stream_decoded_objective_name
    );
    assert_eq!(
        oracle.answer.input_objective_name,
        oracle.answer.decoded_objective_name
    );
    assert_eq!(oracle.answer.input_score, Some(42));
    assert_eq!(oracle.answer.input_score, oracle.answer.stream_decoded_score);
    assert_eq!(oracle.answer.input_score, oracle.answer.decoded_score);
    assert_eq!(oracle.answer.input_display_present, Some(false));
    assert_eq!(
        oracle.answer.input_display_present,
        oracle.answer.stream_decoded_display_present
    );
    assert_eq!(
        oracle.answer.input_number_format_present,
        oracle.answer.stream_decoded_number_format_present
    );

    let mut expected = Vec::new();
    expected.extend_from_slice(&encode_varint("rustmine_owner".len() as i32));
    expected.extend_from_slice(b"rustmine_owner");
    expected.extend_from_slice(&encode_varint("rustmine_objective".len() as i32));
    expected.extend_from_slice(b"rustmine_objective");
    expected.extend_from_slice(&encode_varint(42));
    expected.push(0);
    expected.push(0);
    assert_eq!(
        body, expected,
        "official no-optional set_score fixture should not encode Component or number-format bodies"
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_score")
    .expect("dispatch Play clientbound set_score");

    match decoded {
        packet::Packet::PlaySetScoreClientbound(score) => {
            assert_eq!(score.owner, "rustmine_owner");
            assert_eq!(score.objective_name, "rustmine_objective");
            assert_eq!(score.score.0, 42);
            assert!(!score.display_present);
            assert!(!score.number_format_present);
        }
        other => panic!("decoded packet did not preserve Play clientbound set_score identity: {other:?}"),
    }
    assert!(body_slice.is_empty());
}
