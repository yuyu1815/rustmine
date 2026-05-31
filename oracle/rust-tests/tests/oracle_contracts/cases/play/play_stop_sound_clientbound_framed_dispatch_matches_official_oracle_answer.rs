#[test]
fn play_stop_sound_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_stop_sound_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_stop_sound_clientbound_framed_dispatch_body)
        .expect("spawn play_stop_sound_clientbound oracle stack")
        .join()
        .expect("play_stop_sound_clientbound oracle thread panicked");
}

fn play_stop_sound_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_stop_sound_clientbound_framed_dispatch.test-manifest.json",
        "play_stop_sound_clientbound_framed_dispatch",
        "oracle/contracts/775/play_stop_sound_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_stop_sound_clientbound_framed_dispatch.answer.jsonl",
        "play_stop_sound_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:stop_sound",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundStopSoundPacket")
    );
    assert_eq!(oracle.answer.input_flags, Some(0));
    assert_eq!(oracle.answer.stream_decoded_flags, Some(0));
    assert_eq!(oracle.answer.decoded_flags, Some(0));
    assert_eq!(oracle.answer.stream_decoded_name_present, Some(false));
    assert_eq!(oracle.answer.stream_decoded_source_present, Some(false));
    assert_eq!(oracle.answer.decoded_name_present, Some(false));
    assert_eq!(oracle.answer.decoded_source_present, Some(false));
    assert_eq!(body, vec![0]);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound stop_sound")
    .expect("dispatch Play clientbound stop_sound");

    match decoded {
        packet::Packet::StopSound(stop_sound) => {
            assert_eq!(stop_sound.flags, 0);
            assert!(
                stop_sound.source.is_none(),
                "null/null official stop_sound fixture must not read a source enum"
            );
            assert!(
                stop_sound.sound.is_none(),
                "null/null official stop_sound fixture must not read a sound Identifier"
            );
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound stop_sound identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}
