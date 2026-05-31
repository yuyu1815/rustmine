#[test]
fn play_set_entity_link_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_entity_link_clientbound_framed_dispatch.test-manifest.json",
        "play_set_entity_link_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_entity_link_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_entity_link_clientbound_framed_dispatch.answer.jsonl",
        "play_set_entity_link_clientbound_framed_dispatch_matches_official_oracle_answer",
        "initialized_gametest_framed_dispatch_decode",
        "minecraft:set_entity_link",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetEntityLinkPacket")
    );
    assert_eq!(
        oracle.answer.game_test_fixture.as_deref(),
        Some("rustmine:entity_packet_probe")
    );
    assert_eq!(oracle.answer.source_entity_id, Some(1));
    assert_eq!(oracle.answer.destination_entity_id, Some(2));
    assert_eq!(oracle.answer.stream_decoded_source_entity_id, Some(1));
    assert_eq!(oracle.answer.decoded_source_entity_id, Some(1));
    assert_eq!(oracle.answer.stream_decoded_destination_entity_id, Some(2));
    assert_eq!(oracle.answer.decoded_destination_entity_id, Some(2));
    assert_eq!(body, vec![0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02]);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_entity_link")
    .expect("dispatch Play clientbound set_entity_link");

    match decoded {
        packet::Packet::PlaySetEntityLinkClientbound(link) => {
            assert_eq!(link.source_entity_id, 1);
            assert_eq!(link.destination_entity_id, 2);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound set_entity_link identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());

    let mut unsupported_body: &[u8] =
        &[0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01];
    let err = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut unsupported_body,
    )
    .expect_err("unsupported set_entity_link entity ids must stay unsupported");
    assert!(
        err.to_string()
            .contains("unsupported Play set_entity_link fixture source 2 destination 1"),
        "unexpected error: {err}"
    );
}
