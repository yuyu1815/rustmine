#[test]
fn play_sound_entity_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_sound_entity_clientbound_framed_dispatch.test-manifest.json",
        "play_sound_entity_clientbound_framed_dispatch",
        "oracle/contracts/775/play_sound_entity_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_sound_entity_clientbound_framed_dispatch.answer.jsonl",
        "play_sound_entity_clientbound_framed_dispatch_matches_official_oracle_answer",
        "initialized_gametest_framed_dispatch_decode",
        "minecraft:sound_entity",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSoundEntityPacket")
    );
    assert_eq!(oracle.answer.source_entity_id, Some(1));
    assert_eq!(oracle.answer.input_sound_holder.as_deref(), Some("SoundEvents.AMBIENT_CAVE"));
    assert_eq!(oracle.answer.decoded_sound_location.as_deref(), Some("minecraft:ambient.cave"));
    assert_eq!(oracle.answer.input_source.as_deref(), Some("master"));
    assert_eq!(oracle.answer.input_source, oracle.answer.stream_decoded_source);
    assert_eq!(oracle.answer.input_source, oracle.answer.decoded_source);
    assert_eq!(oracle.answer.stream_decoded_entity_id, Some(1));
    assert_eq!(oracle.answer.decoded_entity_id, Some(1));
    assert_eq!(oracle.answer.input_volume, Some(0.75));
    assert_eq!(oracle.answer.input_volume, oracle.answer.stream_decoded_volume);
    assert_eq!(oracle.answer.input_volume, oracle.answer.decoded_volume);
    assert_eq!(oracle.answer.input_pitch, Some(1.25));
    assert_eq!(oracle.answer.input_pitch, oracle.answer.stream_decoded_pitch);
    assert_eq!(oracle.answer.input_pitch, oracle.answer.decoded_pitch);
    assert_eq!(oracle.answer.input_seed, Some(123456789));
    assert_eq!(oracle.answer.input_seed, oracle.answer.stream_decoded_seed);
    assert_eq!(oracle.answer.input_seed, oracle.answer.decoded_seed);
    assert_eq!(
        body,
        vec![
            0x08, 0x00, 0x01, 0x3f, 0x40, 0x00, 0x00, 0x3f, 0xa0, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x07, 0x5b, 0xcd, 0x15,
        ]
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound sound_entity")
    .expect("dispatch Play clientbound sound_entity");

    match decoded {
        packet::Packet::PlaySoundEntityClientbound(sound) => {
            assert_eq!(sound.sound_holder_id.0, 8);
            assert_eq!(sound.source.0, 0);
            assert_eq!(sound.entity_id.0, 1);
            assert_eq!(sound.volume, 0.75);
            assert_eq!(sound.pitch, 1.25);
            assert_eq!(sound.seed, 123456789);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound sound_entity identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());

    let mut unsupported_body: &[u8] = &[
        0x08, 0x00, 0x02, 0x3f, 0x40, 0x00, 0x00, 0x3f, 0xa0, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x07, 0x5b, 0xcd, 0x15,
    ];
    let err = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut unsupported_body,
    )
    .expect_err("unsupported sound_entity entity fixture id must stay unsupported");
    assert!(
        err.to_string()
            .contains("unsupported Play sound_entity fixture entity id 2"),
        "unexpected error: {err}"
    );
}
