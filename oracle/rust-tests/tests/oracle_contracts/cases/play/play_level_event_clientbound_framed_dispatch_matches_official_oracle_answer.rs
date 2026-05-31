#[test]
fn play_level_event_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_level_event_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_level_event_clientbound_framed_dispatch_body)
        .expect("spawn play_level_event_clientbound oracle stack")
        .join()
        .expect("play_level_event_clientbound oracle thread panicked");
}

fn play_level_event_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_level_event_clientbound_framed_dispatch.test-manifest.json",
        "play_level_event_clientbound_framed_dispatch",
        "oracle/contracts/775/play_level_event_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_level_event_clientbound_framed_dispatch.answer.jsonl",
        "play_level_event_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:level_event",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundLevelEventPacket")
    );
    assert_eq!(
        oracle.answer.input_level_event_type,
        oracle.answer.decoded_level_event_type
    );
    assert_eq!(
        oracle.answer.stream_decoded_level_event_type,
        oracle.answer.decoded_level_event_type
    );
    assert_eq!(oracle.answer.input_block_x, oracle.answer.decoded_block_x);
    assert_eq!(
        oracle.answer.stream_decoded_block_x,
        oracle.answer.decoded_block_x
    );
    assert_eq!(oracle.answer.input_block_y, oracle.answer.decoded_block_y);
    assert_eq!(
        oracle.answer.stream_decoded_block_y,
        oracle.answer.decoded_block_y
    );
    assert_eq!(oracle.answer.input_block_z, oracle.answer.decoded_block_z);
    assert_eq!(
        oracle.answer.stream_decoded_block_z,
        oracle.answer.decoded_block_z
    );
    assert_eq!(oracle.answer.input_data, oracle.answer.decoded_data);
    assert_eq!(
        oracle.answer.stream_decoded_data,
        oracle.answer.decoded_data
    );
    assert_eq!(
        oracle.answer.input_global_event,
        oracle.answer.decoded_global_event
    );
    assert_eq!(
        oracle.answer.stream_decoded_global_event,
        oracle.answer.decoded_global_event
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound level_event")
    .expect("dispatch Play clientbound level_event");

    match decoded {
        packet::Packet::Effect(effect) => {
            assert_eq!(
                effect.effect_id,
                oracle.answer.decoded_level_event_type.unwrap()
            );
            assert_eq!(effect.location.x, oracle.answer.decoded_block_x.unwrap());
            assert_eq!(effect.location.y, oracle.answer.decoded_block_y.unwrap());
            assert_eq!(effect.location.z, oracle.answer.decoded_block_z.unwrap());
            assert_eq!(effect.data, oracle.answer.decoded_data.unwrap());
            assert_eq!(
                effect.disable_relative,
                oracle.answer.decoded_global_event.unwrap()
            );
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound level_event body: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}

