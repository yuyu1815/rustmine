#[test]
fn play_game_event_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_game_event_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_game_event_clientbound_framed_dispatch_body)
        .expect("spawn play_game_event_clientbound oracle stack")
        .join()
        .expect("play_game_event_clientbound oracle thread panicked");
}

fn play_game_event_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_game_event_clientbound_framed_dispatch.test-manifest.json",
        "play_game_event_clientbound_framed_dispatch",
        "oracle/contracts/775/play_game_event_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_game_event_clientbound_framed_dispatch.answer.jsonl",
        "play_game_event_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:game_event",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundGameEventPacket")
    );
    assert_eq!(
        oracle.answer.input_game_event,
        oracle.answer.decoded_game_event
    );
    assert_eq!(
        oracle.answer.stream_decoded_game_event,
        oracle.answer.decoded_game_event
    );
    assert_eq!(
        oracle.answer.input_game_event_id,
        oracle.answer.decoded_game_event_id
    );
    assert_eq!(
        oracle.answer.stream_decoded_game_event_id,
        oracle.answer.decoded_game_event_id
    );
    assert_eq!(oracle.answer.input_param, oracle.answer.decoded_param);
    assert_eq!(
        oracle.answer.stream_decoded_param,
        oracle.answer.decoded_param
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound game_event")
    .expect("dispatch Play clientbound game_event");

    match decoded {
        packet::Packet::PlayGameEventClientbound(game_event) => {
            assert_eq!(
                game_event.event as i32,
                oracle.answer.decoded_game_event_id.unwrap()
            );
            assert_eq!(game_event.param, oracle.answer.decoded_param.unwrap());
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound game_event identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}

