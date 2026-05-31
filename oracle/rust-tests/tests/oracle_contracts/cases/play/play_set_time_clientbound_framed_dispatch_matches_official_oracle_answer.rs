#[test]
fn play_set_time_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_set_time_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_set_time_clientbound_framed_dispatch_body)
        .expect("spawn play_set_time_clientbound oracle stack")
        .join()
        .expect("play_set_time_clientbound oracle thread panicked");
}

fn play_set_time_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_time_clientbound_framed_dispatch.test-manifest.json",
        "play_set_time_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_time_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_time_clientbound_framed_dispatch.answer.jsonl",
        "play_set_time_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_time",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetTimePacket")
    );
    assert_eq!(oracle.answer.input_game_time, Some(123456789));
    assert_eq!(
        oracle.answer.input_game_time,
        oracle.answer.stream_decoded_game_time
    );
    assert_eq!(oracle.answer.input_game_time, oracle.answer.decoded_game_time);
    assert_eq!(oracle.answer.input_clock_update_count, Some(0));
    assert_eq!(
        oracle.answer.input_clock_update_count,
        oracle.answer.stream_decoded_clock_update_count
    );
    assert_eq!(
        oracle.answer.input_clock_update_count,
        oracle.answer.decoded_clock_update_count
    );

    let mut expected = 123456789i64.to_be_bytes().to_vec();
    expected.extend_from_slice(&encode_varint(0));
    assert_eq!(
        body, expected,
        "official empty-clock-update set_time fixture should encode gameTime and a zero map count"
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_time")
    .expect("dispatch Play clientbound set_time");

    match decoded {
        packet::Packet::PlaySetTimeClientbound(time) => {
            assert_eq!(time.game_time, 123456789);
            assert_eq!(time.clock_update_count.0, 0);
        }
        other => panic!("decoded packet did not preserve Play clientbound set_time identity: {other:?}"),
    }
    assert!(body_slice.is_empty());
}
