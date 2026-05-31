#[test]
fn play_waypoint_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_waypoint_clientbound_framed_dispatch.test-manifest.json",
        "play_waypoint_clientbound_framed_dispatch",
        "oracle/contracts/775/play_waypoint_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_waypoint_clientbound_framed_dispatch.answer.jsonl",
        "play_waypoint_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:waypoint",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundTrackedWaypointPacket")
    );
    assert_eq!(
        oracle.answer.input_waypoint_uuid.as_deref(),
        Some("00000000-0000-0000-0000-000000000123")
    );
    assert_eq!(oracle.answer.decoded_operation.as_deref(), Some("UNTRACK"));
    assert_eq!(
        oracle.answer.decoded_waypoint_id.as_deref(),
        Some("Left[00000000-0000-0000-0000-000000000123]")
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound waypoint")
    .expect("dispatch Play clientbound waypoint");

    match decoded {
        packet::Packet::PlayWaypointClientbound(waypoint) => {
            assert_eq!(waypoint.operation_id.0, 1);
            assert_eq!(
                waypoint.waypoint_payload,
                decode_hex(
                    "0100000000000000000000000000000123116d696e6563726166743a64656661756c740000",
                    "expected waypoint remove payload",
                )
            );
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound waypoint identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}
