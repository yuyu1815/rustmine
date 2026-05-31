#[test]
fn play_update_attributes_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_update_attributes_clientbound_framed_dispatch.test-manifest.json",
        "play_update_attributes_clientbound_framed_dispatch",
        "oracle/contracts/775/play_update_attributes_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_update_attributes_clientbound_framed_dispatch.answer.jsonl",
        "play_update_attributes_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:update_attributes",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundUpdateAttributesPacket")
    );
    assert_eq!(oracle.answer.input_entity_id, Some(12345));
    assert_eq!(
        oracle.answer.input_entity_id,
        oracle.answer.stream_decoded_entity_id
    );
    assert_eq!(oracle.answer.input_entity_id, oracle.answer.decoded_entity_id);
    assert_eq!(oracle.answer.input_attribute_count, Some(0));
    assert_eq!(
        oracle.answer.input_attribute_count,
        oracle.answer.stream_decoded_attribute_count
    );
    assert_eq!(
        oracle.answer.input_attribute_count,
        oracle.answer.decoded_attribute_count
    );
    assert_eq!(body, vec![0xb9, 0x60, 0x00]);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound update_attributes")
    .expect("dispatch Play clientbound update_attributes");

    match decoded {
        packet::Packet::PlayUpdateAttributesClientbound(attributes) => {
            assert_eq!(attributes.entity_id.0, 12345);
            assert_eq!(attributes.attribute_count.0, 0);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound update_attributes identity: {other:?}"
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
    .expect_err("non-empty attribute list must stay unsupported");
    assert!(
        err.to_string()
            .contains("unsupported non-empty Play update_attributes attribute count 1"),
        "unexpected error: {err}"
    );
}
