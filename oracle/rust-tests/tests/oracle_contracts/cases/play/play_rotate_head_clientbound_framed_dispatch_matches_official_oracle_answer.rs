#[test]
fn play_rotate_head_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_rotate_head_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_rotate_head_clientbound_framed_dispatch_body)
        .expect("spawn play_rotate_head_clientbound oracle stack")
        .join()
        .expect("play_rotate_head_clientbound oracle thread panicked");
}

fn play_rotate_head_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_rotate_head_clientbound_framed_dispatch.test-manifest.json",
        "play_rotate_head_clientbound_framed_dispatch",
        "oracle/contracts/775/play_rotate_head_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_rotate_head_clientbound_framed_dispatch.answer.jsonl",
        "play_rotate_head_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:rotate_head",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundRotateHeadPacket")
    );
    assert_eq!(
        oracle.answer.decoded_entity_id,
        oracle.answer.input_entity_id
    );
    assert_eq!(
        oracle.answer.stream_decoded_entity_id,
        oracle.answer.input_entity_id
    );
    assert_eq!(
        oracle.answer.decoded_y_head_rot_byte,
        oracle.answer.input_y_head_rot_byte
    );
    assert_eq!(
        oracle.answer.stream_decoded_y_head_rot_byte,
        oracle.answer.input_y_head_rot_byte
    );
    assert_eq!(
        oracle.answer.stream_decoded_y_head_rot_degrees,
        oracle.answer.decoded_y_head_rot_degrees
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound rotate_head")
    .expect("dispatch Play clientbound rotate_head");

    match decoded {
        packet::Packet::EntityHeadLook(head) => {
            assert_eq!(head.entity_id.0, oracle.answer.decoded_entity_id.unwrap());
            assert_eq!(
                head.head_yaw,
                oracle.answer.decoded_y_head_rot_byte.unwrap()
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound rotate_head identity: {other:?}"
            )
        }
    }
    assert!(body_slice.is_empty());
}

