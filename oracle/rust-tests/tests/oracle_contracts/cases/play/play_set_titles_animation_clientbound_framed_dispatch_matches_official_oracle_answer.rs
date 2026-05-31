#[test]
fn play_set_titles_animation_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_set_titles_animation_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_set_titles_animation_clientbound_framed_dispatch_body)
        .expect("spawn play_set_titles_animation_clientbound oracle stack")
        .join()
        .expect("play_set_titles_animation_clientbound oracle thread panicked");
}

fn play_set_titles_animation_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_titles_animation_clientbound_framed_dispatch.test-manifest.json",
        "play_set_titles_animation_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_titles_animation_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_titles_animation_clientbound_framed_dispatch.answer.jsonl",
        "play_set_titles_animation_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_titles_animation",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetTitlesAnimationPacket")
    );
    assert_eq!(oracle.answer.stream_decoded_fade_in, oracle.answer.decoded_fade_in);
    assert_eq!(oracle.answer.stream_decoded_stay, oracle.answer.decoded_stay);
    assert_eq!(oracle.answer.stream_decoded_fade_out, oracle.answer.decoded_fade_out);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_titles_animation")
    .expect("dispatch Play clientbound set_titles_animation");

    match decoded {
        packet::Packet::PlaySetTitlesAnimationClientbound(titles_animation) => {
            assert_eq!(titles_animation.fade_in, oracle.answer.decoded_fade_in.unwrap());
            assert_eq!(titles_animation.stay, oracle.answer.decoded_stay.unwrap());
            assert_eq!(titles_animation.fade_out, oracle.answer.decoded_fade_out.unwrap());
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound set_titles_animation identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}
