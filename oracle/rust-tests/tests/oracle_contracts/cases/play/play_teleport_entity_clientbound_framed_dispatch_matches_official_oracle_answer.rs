#[test]
fn play_teleport_entity_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_teleport_entity_clientbound_framed_dispatch.test-manifest.json",
        "play_teleport_entity_clientbound_framed_dispatch",
        "oracle/contracts/775/play_teleport_entity_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_teleport_entity_clientbound_framed_dispatch.answer.jsonl",
        "play_teleport_entity_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:teleport_entity",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundTeleportEntityPacket")
    );
    assert_eq!(oracle.answer.input_entity_id, Some(12345));
    assert_eq!(oracle.answer.input_entity_id, oracle.answer.decoded_entity_id);
    assert_eq!(oracle.answer.decoded_position_x, Some(1.25));
    assert_eq!(oracle.answer.decoded_position_y, Some(2.5));
    assert_eq!(oracle.answer.decoded_position_z, Some(-3.75));
    assert_eq!(oracle.answer.decoded_relative_count, Some(0));
    assert_eq!(oracle.answer.decoded_on_ground, Some(false));

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound teleport_entity")
    .expect("dispatch Play clientbound teleport_entity");

    match decoded {
        packet::Packet::PlayTeleportEntityClientbound(teleport_entity) => {
            assert_eq!(teleport_entity.entity_id.0, 12345);
            assert_eq!(teleport_entity.position_x, 1.25);
            assert_eq!(teleport_entity.position_y, 2.5);
            assert_eq!(teleport_entity.position_z, -3.75);
            assert_eq!(teleport_entity.delta_x, 0.0);
            assert_eq!(teleport_entity.delta_y, 0.0);
            assert_eq!(teleport_entity.delta_z, 0.0);
            assert_eq!(teleport_entity.y_rot, 45.0);
            assert_eq!(teleport_entity.x_rot, 10.0);
            assert_eq!(teleport_entity.relative_mask, 0);
            assert!(!teleport_entity.on_ground);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound teleport_entity identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}
