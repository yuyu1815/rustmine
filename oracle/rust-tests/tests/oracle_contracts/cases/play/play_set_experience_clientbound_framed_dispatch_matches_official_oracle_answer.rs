#[test]
fn play_set_experience_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_set_experience_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_set_experience_clientbound_framed_dispatch_body)
        .expect("spawn play_set_experience_clientbound oracle stack")
        .join()
        .expect("play_set_experience_clientbound oracle thread panicked");
}

fn play_set_experience_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_experience_clientbound_framed_dispatch.test-manifest.json",
        "play_set_experience_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_experience_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_experience_clientbound_framed_dispatch.answer.jsonl",
        "play_set_experience_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_experience",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetExperiencePacket")
    );
    let official_experience_progress = f32::from_be_bytes([body[0], body[1], body[2], body[3]]);
    let (official_level, level_len) = read_varint_prefix(&body[4..]);
    let (official_total_experience, total_len) = read_varint_prefix(&body[(4 + level_len)..]);
    assert_eq!(4 + level_len + total_len, body.len());

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_experience")
    .expect("dispatch Play clientbound set_experience");

    match decoded {
        packet::Packet::SetExperience(experience) => {
            assert_eq!(experience.experience_bar, official_experience_progress);
            assert_eq!(experience.level.0, official_level);
            assert_eq!(experience.total_experience.0, official_total_experience);
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound set_experience identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}
