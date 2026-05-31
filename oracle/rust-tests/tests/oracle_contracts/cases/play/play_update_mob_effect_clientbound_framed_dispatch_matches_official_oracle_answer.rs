#[test]
fn play_update_mob_effect_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_update_mob_effect_clientbound_framed_dispatch.test-manifest.json",
        "play_update_mob_effect_clientbound_framed_dispatch",
        "oracle/contracts/775/play_update_mob_effect_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_update_mob_effect_clientbound_framed_dispatch.answer.jsonl",
        "play_update_mob_effect_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:update_mob_effect",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundUpdateMobEffectPacket")
    );
    assert_eq!(oracle.answer.input_entity_id, Some(12345));
    assert_eq!(oracle.answer.input_entity_id, oracle.answer.stream_decoded_entity_id);
    assert_eq!(oracle.answer.input_entity_id, oracle.answer.decoded_entity_id);
    assert_eq!(oracle.answer.input_effect_holder.as_deref(), Some("MobEffects.SPEED"));
    assert_eq!(
        oracle.answer.decoded_effect_description_id.as_deref(),
        Some("effect.minecraft.speed")
    );
    assert_eq!(oracle.answer.input_amplifier, Some(1));
    assert_eq!(oracle.answer.input_amplifier, oracle.answer.stream_decoded_amplifier);
    assert_eq!(oracle.answer.input_amplifier, oracle.answer.decoded_amplifier);
    assert_eq!(oracle.answer.input_effect_duration, Some(200));
    assert_eq!(
        oracle.answer.input_effect_duration,
        oracle.answer.stream_decoded_effect_duration
    );
    assert_eq!(oracle.answer.input_effect_duration, oracle.answer.decoded_effect_duration);
    assert_eq!(oracle.answer.input_ambient, Some(false));
    assert_eq!(oracle.answer.input_visible, Some(true));
    assert_eq!(oracle.answer.input_show_icon, Some(true));
    assert_eq!(oracle.answer.input_blend, Some(false));
    assert_eq!(body, vec![0xb9, 0x60, 0x00, 0x01, 0xc8, 0x01, 0x06]);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound update_mob_effect")
    .expect("dispatch Play clientbound update_mob_effect");

    match decoded {
        packet::Packet::PlayUpdateMobEffectClientbound(effect) => {
            assert_eq!(effect.entity_id.0, 12345);
            assert_eq!(effect.effect_holder_id.0, 0);
            assert_eq!(effect.amplifier.0, 1);
            assert_eq!(effect.duration.0, 200);
            assert_eq!(effect.flags, 0x06);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound update_mob_effect identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());

    let mut unsupported_body: &[u8] = &[0xb9, 0x60, 0x01];
    let err = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut unsupported_body,
    )
    .expect_err("unsupported MobEffect holder must stay unsupported");
    assert!(
        err.to_string()
            .contains("unsupported Play update_mob_effect MobEffect holder id 1"),
        "unexpected error: {err}"
    );
}
