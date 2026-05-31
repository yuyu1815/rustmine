#[test]
fn play_set_passengers_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_passengers_clientbound_framed_dispatch.test-manifest.json",
        "play_set_passengers_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_passengers_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_passengers_clientbound_framed_dispatch.answer.jsonl",
        "play_set_passengers_clientbound_framed_dispatch_matches_official_oracle_answer",
        "initialized_gametest_framed_dispatch_decode",
        "minecraft:set_passengers",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetPassengersPacket")
    );
    assert_eq!(oracle.answer.vehicle_entity_id, Some(3));
    assert_eq!(oracle.answer.passenger_entity_id, Some(4));
    assert_eq!(oracle.answer.vehicle_passenger_count, Some(1));
    assert_eq!(oracle.answer.stream_decoded_vehicle_entity_id, Some(3));
    assert_eq!(oracle.answer.decoded_vehicle_entity_id, Some(3));
    assert_eq!(
        oracle.answer.stream_decoded_passenger_entity_ids.as_deref(),
        Some([4].as_slice())
    );
    assert_eq!(
        oracle.answer.decoded_passenger_entity_ids.as_deref(),
        Some([4].as_slice())
    );
    assert_eq!(body, vec![0x03, 0x01, 0x04]);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_passengers")
    .expect("dispatch Play clientbound set_passengers");

    match decoded {
        packet::Packet::PlaySetPassengersClientbound(passengers) => {
            assert_eq!(passengers.vehicle_entity_id.0, 3);
            assert_eq!(passengers.passenger_entity_ids.data.len(), 1);
            assert_eq!(passengers.passenger_entity_ids.data[0].0, 4);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound set_passengers identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());

    let mut unsupported_body: &[u8] = &[0x03, 0x02, 0x04, 0x05];
    let err = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut unsupported_body,
    )
    .expect_err("unsupported set_passengers passenger topology must stay unsupported");
    assert!(
        err.to_string()
            .contains("unsupported Play set_passengers fixture vehicle 3 passengers [4, 5]"),
        "unexpected error: {err}"
    );
}
