#[test]
fn play_ticking_step_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_ticking_step_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_ticking_step_clientbound_framed_dispatch_body)
        .expect("spawn play_ticking_step_clientbound oracle stack")
        .join()
        .expect("play_ticking_step_clientbound oracle thread panicked");
}

fn play_ticking_step_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_ticking_step_clientbound_framed_dispatch.test-manifest.json",
        "play_ticking_step_clientbound_framed_dispatch",
        "oracle/contracts/775/play_ticking_step_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_ticking_step_clientbound_framed_dispatch.answer.jsonl",
        "play_ticking_step_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:ticking_step",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundTickingStepPacket")
    );
    assert_eq!(oracle.answer.input_tick_steps, oracle.answer.stream_decoded_tick_steps);
    assert_eq!(oracle.answer.input_tick_steps, oracle.answer.decoded_tick_steps);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound ticking_step")
    .expect("dispatch Play clientbound ticking_step");

    match decoded {
        packet::Packet::PlayTickingStepClientbound(ticking_step) => {
            assert_eq!(
                Some(ticking_step.tick_steps.0),
                oracle.answer.decoded_tick_steps,
                "decoded Play ticking_step field differs from official answer"
            );
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound ticking_step identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}
