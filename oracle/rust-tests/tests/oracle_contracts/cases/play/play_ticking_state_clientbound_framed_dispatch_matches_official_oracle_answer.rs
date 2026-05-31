#[test]
fn play_ticking_state_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_ticking_state_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_ticking_state_clientbound_framed_dispatch_body)
        .expect("spawn play_ticking_state_clientbound oracle stack")
        .join()
        .expect("play_ticking_state_clientbound oracle thread panicked");
}

fn play_ticking_state_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_ticking_state_clientbound_framed_dispatch.test-manifest.json",
        "play_ticking_state_clientbound_framed_dispatch",
        "oracle/contracts/775/play_ticking_state_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_ticking_state_clientbound_framed_dispatch.answer.jsonl",
        "play_ticking_state_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:ticking_state",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundTickingStatePacket")
    );
    assert_eq!(oracle.answer.input_tick_rate, oracle.answer.stream_decoded_tick_rate);
    assert_eq!(oracle.answer.input_tick_rate, oracle.answer.decoded_tick_rate);
    assert_eq!(oracle.answer.input_frozen, oracle.answer.stream_decoded_frozen);
    assert_eq!(oracle.answer.input_frozen, oracle.answer.decoded_frozen);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound ticking_state")
    .expect("dispatch Play clientbound ticking_state");

    match decoded {
        packet::Packet::PlayTickingStateClientbound(ticking_state) => {
            assert_eq!(
                Some(ticking_state.tick_rate),
                oracle.answer.decoded_tick_rate,
                "decoded Play ticking_state tick_rate differs from official answer"
            );
            assert_eq!(
                Some(ticking_state.frozen),
                oracle.answer.decoded_frozen,
                "decoded Play ticking_state frozen flag differs from official answer"
            );
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound ticking_state identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}
