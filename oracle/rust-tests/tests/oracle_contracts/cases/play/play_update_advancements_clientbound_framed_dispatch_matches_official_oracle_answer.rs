#[test]
fn play_update_advancements_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_update_advancements_clientbound_framed_dispatch.test-manifest.json",
        "play_update_advancements_clientbound_framed_dispatch",
        "oracle/contracts/775/play_update_advancements_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_update_advancements_clientbound_framed_dispatch.answer.jsonl",
        "play_update_advancements_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:update_advancements",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundUpdateAdvancementsPacket")
    );
    assert_eq!(oracle.answer.input_reset, Some(false));
    assert_eq!(oracle.answer.input_reset, oracle.answer.decoded_reset);
    assert_eq!(oracle.answer.input_show_advancements, Some(false));
    assert_eq!(
        oracle.answer.input_show_advancements,
        oracle.answer.decoded_show_advancements
    );
    assert_eq!(oracle.answer.decoded_added_count, Some(0));
    assert_eq!(oracle.answer.decoded_removed_count, Some(0));
    assert_eq!(oracle.answer.decoded_progress_count, Some(0));
    assert_eq!(body, decode_hex("0000000000", "expected update_advancements body"));

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound update_advancements")
    .expect("dispatch Play clientbound update_advancements");

    match decoded {
        packet::Packet::PlayUpdateAdvancementsClientbound(update_advancements) => {
            assert!(!update_advancements.reset);
            assert_eq!(update_advancements.added_count.0, 0);
            assert_eq!(update_advancements.removed_count.0, 0);
            assert_eq!(update_advancements.progress_count.0, 0);
            assert!(!update_advancements.show_advancements);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound update_advancements identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}
