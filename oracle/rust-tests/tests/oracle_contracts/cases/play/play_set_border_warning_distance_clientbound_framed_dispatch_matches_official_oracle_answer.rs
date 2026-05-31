#[test]
fn play_set_border_warning_distance_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_set_border_warning_distance_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_set_border_warning_distance_clientbound_framed_dispatch_body)
        .expect("spawn play_set_border_warning_distance_clientbound oracle stack")
        .join()
        .expect("play_set_border_warning_distance_clientbound oracle thread panicked");
}

fn play_set_border_warning_distance_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_border_warning_distance_clientbound_framed_dispatch.test-manifest.json",
        "play_set_border_warning_distance_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_border_warning_distance_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_border_warning_distance_clientbound_framed_dispatch.answer.jsonl",
        "play_set_border_warning_distance_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_border_warning_distance",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetBorderWarningDistancePacket")
    );
    assert_eq!(
        oracle.answer.stream_decoded_warning_blocks,
        oracle.answer.input_warning_blocks
    );
    assert_eq!(
        oracle.answer.decoded_warning_blocks,
        oracle.answer.input_warning_blocks
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_border_warning_distance")
    .expect("dispatch Play clientbound set_border_warning_distance");

    match decoded {
        packet::Packet::PlaySetBorderWarningDistanceClientbound(border) => {
            assert_eq!(
                border.warning_blocks.0,
                oracle.answer.decoded_warning_blocks.unwrap()
            );
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound set_border_warning_distance identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}

