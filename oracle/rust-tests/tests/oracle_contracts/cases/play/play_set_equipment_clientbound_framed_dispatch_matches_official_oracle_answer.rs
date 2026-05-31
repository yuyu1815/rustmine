#[test]
fn play_set_equipment_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_equipment_clientbound_framed_dispatch.test-manifest.json",
        "play_set_equipment_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_equipment_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_equipment_clientbound_framed_dispatch.answer.jsonl",
        "play_set_equipment_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_equipment",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetEquipmentPacket")
    );
    assert_eq!(oracle.answer.input_entity_id, Some(123));
    assert_eq!(oracle.answer.input_entity_id, oracle.answer.stream_decoded_entity_id);
    assert_eq!(oracle.answer.input_entity_id, oracle.answer.decoded_entity_id);
    assert_eq!(oracle.answer.input_equipment_slot.as_deref(), Some("MAINHAND"));
    assert_eq!(
        oracle.answer.input_equipment_slot,
        oracle.answer.stream_decoded_equipment_slot
    );
    assert_eq!(
        oracle.answer.input_equipment_slot,
        oracle.answer.decoded_equipment_slot
    );
    assert_eq!(oracle.answer.input_equipment_slot_ordinal, Some(0));
    assert_eq!(oracle.answer.input_equipment_entry_count, Some(1));
    assert_eq!(oracle.answer.input_item_empty, Some(true));
    assert_eq!(body, vec![0x7b, 0x00, 0x00]);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_equipment")
    .expect("dispatch Play clientbound set_equipment");

    match decoded {
        packet::Packet::PlaySetEquipmentClientbound(equipment) => {
            assert_eq!(equipment.entity_id.0, 123);
            assert_eq!(equipment.equipment_slot, 0);
            assert!(equipment.item.is_none());
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound set_equipment identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());

    let mut multi_entry = [0x7b, 0x80, 0x00].as_slice();
    let err = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut multi_entry,
    )
    .expect_err("multi-entry set_equipment fixture should be rejected");
    assert!(
        err.to_string().contains("unsupported multi-entry Play set_equipment slot byte 128"),
        "unexpected rejection: {err}"
    );
}
