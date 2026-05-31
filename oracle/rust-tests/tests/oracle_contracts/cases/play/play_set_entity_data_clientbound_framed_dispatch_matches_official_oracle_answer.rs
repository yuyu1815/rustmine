#[test]
fn play_set_entity_data_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_entity_data_clientbound_framed_dispatch.test-manifest.json",
        "play_set_entity_data_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_entity_data_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_entity_data_clientbound_framed_dispatch.answer.jsonl",
        "play_set_entity_data_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_entity_data",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetEntityDataPacket")
    );
    assert_eq!(oracle.answer.input_entity_id, Some(12345));
    assert_eq!(
        oracle.answer.input_entity_id,
        oracle.answer.stream_decoded_entity_id
    );
    assert_eq!(oracle.answer.input_entity_id, oracle.answer.decoded_entity_id);
    assert_eq!(oracle.answer.input_packed_item_count, Some(0));
    assert_eq!(
        oracle.answer.input_packed_item_count,
        oracle.answer.stream_decoded_packed_item_count
    );
    assert_eq!(
        oracle.answer.input_packed_item_count,
        oracle.answer.decoded_packed_item_count
    );
    assert_eq!(body, vec![0xb9, 0x60, 0xff]);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_entity_data")
    .expect("dispatch Play clientbound set_entity_data");

    match decoded {
        packet::Packet::PlaySetEntityDataClientbound(entity_data) => {
            assert_eq!(entity_data.entity_id.0, 12345);
            assert_eq!(entity_data.packed_item_count.0, 0);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound set_entity_data identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());

    let mut unsupported_body: &[u8] = &[0xb9, 0x60, 0x00];
    let err = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut unsupported_body,
    )
    .expect_err("non-empty metadata marker must stay unsupported");
    assert!(
        err.to_string()
            .contains("unsupported non-empty Play set_entity_data metadata marker 0"),
        "unexpected error: {err}"
    );
}
