#[test]
fn play_set_player_inventory_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_player_inventory_clientbound_framed_dispatch.test-manifest.json",
        "play_set_player_inventory_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_player_inventory_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_player_inventory_clientbound_framed_dispatch.answer.jsonl",
        "play_set_player_inventory_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_player_inventory",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetPlayerInventoryPacket")
    );
    assert_eq!(oracle.answer.input_slot, Some(7));
    assert_eq!(oracle.answer.input_slot, oracle.answer.stream_decoded_slot);
    assert_eq!(oracle.answer.input_slot, oracle.answer.decoded_slot);
    assert_eq!(oracle.answer.input_item_empty, Some(true));
    assert_eq!(
        oracle.answer.input_item_empty,
        oracle.answer.stream_decoded_item_empty
    );
    assert_eq!(oracle.answer.input_item_empty, oracle.answer.decoded_item_empty);
    assert_eq!(body, vec![7, 0]);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_player_inventory")
    .expect("dispatch Play clientbound set_player_inventory");

    match decoded {
        packet::Packet::PlaySetPlayerInventoryClientbound(inventory) => {
            assert_eq!(inventory.slot.0, 7);
            assert!(inventory.item.is_none());
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound set_player_inventory identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}
