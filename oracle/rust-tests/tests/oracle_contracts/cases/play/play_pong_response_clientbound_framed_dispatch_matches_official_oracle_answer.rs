#[test]
fn play_pong_response_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_pong_response_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_pong_response_clientbound_framed_dispatch_body)
        .expect("spawn play_pong_response_clientbound oracle stack")
        .join()
        .expect("play_pong_response_clientbound oracle thread panicked");
}

fn play_pong_response_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_pong_response_clientbound_framed_dispatch.test-manifest.json",
        "play_pong_response_clientbound_framed_dispatch",
        "oracle/contracts/775/play_pong_response_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_pong_response_clientbound_framed_dispatch.answer.jsonl",
        "play_pong_response_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:pong_response",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.ping.ClientboundPongResponsePacket")
    );
    assert_eq!(oracle.answer.decoded_time, oracle.answer.input_time);
    assert_eq!(oracle.answer.stream_decoded_time, oracle.answer.input_time);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound pong_response")
    .expect("dispatch Play clientbound pong_response");

    match decoded {
        packet::Packet::PlayPongResponseClientbound_i64(pong) => {
            assert_eq!(pong.time, oracle.answer.decoded_time.unwrap());
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound pong_response identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}

