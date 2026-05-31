#[test]
fn play_projectile_power_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_projectile_power_clientbound_framed_dispatch.test-manifest.json",
        "play_projectile_power_clientbound_framed_dispatch",
        "oracle/contracts/775/play_projectile_power_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_projectile_power_clientbound_framed_dispatch.answer.jsonl",
        "play_projectile_power_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:projectile_power",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundProjectilePowerPacket")
    );
    assert_eq!(oracle.answer.input_entity_id, Some(12345));
    assert_eq!(
        oracle.answer.input_entity_id,
        oracle.answer.stream_decoded_entity_id
    );
    assert_eq!(oracle.answer.input_entity_id, oracle.answer.decoded_entity_id);
    assert_eq!(oracle.answer.input_acceleration_power, Some(2.5));
    assert_eq!(
        oracle.answer.input_acceleration_power,
        oracle.answer.stream_decoded_acceleration_power
    );
    assert_eq!(
        oracle.answer.input_acceleration_power,
        oracle.answer.decoded_acceleration_power
    );
    assert_eq!(
        body,
        vec![0xb9, 0x60, 0x40, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound projectile_power")
    .expect("dispatch Play clientbound projectile_power");

    match decoded {
        packet::Packet::PlayProjectilePowerClientbound(projectile_power) => {
            assert_eq!(projectile_power.entity_id.0, 12345);
            assert_eq!(projectile_power.acceleration_power, 2.5);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound projectile_power identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}
