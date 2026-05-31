#[test]
fn play_set_border_lerp_size_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_set_border_lerp_size_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_set_border_lerp_size_clientbound_framed_dispatch_body)
        .expect("spawn play_set_border_lerp_size_clientbound oracle stack")
        .join()
        .expect("play_set_border_lerp_size_clientbound oracle thread panicked");
}

fn play_set_border_lerp_size_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_border_lerp_size_clientbound_framed_dispatch.test-manifest.json",
        "play_set_border_lerp_size_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_border_lerp_size_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_border_lerp_size_clientbound_framed_dispatch.answer.jsonl",
        "play_set_border_lerp_size_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_border_lerp_size",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetBorderLerpSizePacket")
    );
    assert_eq!(
        oracle.answer.stream_decoded_old_size,
        oracle.answer.input_old_size
    );
    assert_eq!(oracle.answer.decoded_old_size, oracle.answer.input_old_size);
    assert_eq!(
        oracle.answer.stream_decoded_new_size,
        oracle.answer.input_new_size
    );
    assert_eq!(oracle.answer.decoded_new_size, oracle.answer.input_new_size);
    assert_eq!(
        oracle.answer.stream_decoded_lerp_time,
        oracle.answer.input_lerp_time
    );
    assert_eq!(
        oracle.answer.decoded_lerp_time,
        oracle.answer.input_lerp_time
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_border_lerp_size")
    .expect("dispatch Play clientbound set_border_lerp_size");

    match decoded {
        packet::Packet::PlaySetBorderLerpSizeClientbound(border) => {
            assert_eq!(border.old_size, oracle.answer.decoded_old_size.unwrap());
            assert_eq!(border.new_size, oracle.answer.decoded_new_size.unwrap());
            assert_eq!(border.lerp_time.0, oracle.answer.decoded_lerp_time.unwrap());
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound set_border_lerp_size identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}

