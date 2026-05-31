#[test]
fn play_hurt_animation_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_hurt_animation_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_hurt_animation_clientbound_framed_dispatch_body)
        .expect("spawn play_hurt_animation_clientbound oracle stack")
        .join()
        .expect("play_hurt_animation_clientbound oracle thread panicked");
}

fn play_hurt_animation_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_hurt_animation_clientbound_framed_dispatch.test-manifest.json",
        "play_hurt_animation_clientbound_framed_dispatch",
        "oracle/contracts/775/play_hurt_animation_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_hurt_animation_clientbound_framed_dispatch.answer.jsonl",
        "play_hurt_animation_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:hurt_animation",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundHurtAnimationPacket")
    );
    assert_eq!(
        oracle.answer.input_entity_id,
        oracle.answer.stream_decoded_entity_id
    );
    assert_eq!(
        oracle.answer.input_entity_id,
        oracle.answer.decoded_entity_id
    );
    assert_eq!(oracle.answer.input_yaw, oracle.answer.stream_decoded_yaw);
    assert_eq!(oracle.answer.input_yaw, oracle.answer.decoded_yaw);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound hurt_animation")
    .expect("dispatch Play clientbound hurt_animation");

    match decoded {
        packet::Packet::PlayHurtAnimationClientbound(hurt) => {
            assert_eq!(hurt.entity_id.0, oracle.answer.decoded_entity_id.unwrap());
            assert_eq!(hurt.yaw, oracle.answer.decoded_yaw.unwrap());
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound hurt_animation identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}

