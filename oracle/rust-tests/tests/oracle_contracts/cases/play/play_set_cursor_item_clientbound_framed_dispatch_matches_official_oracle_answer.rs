#[test]
fn play_set_cursor_item_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_cursor_item_clientbound_framed_dispatch.test-manifest.json",
        "play_set_cursor_item_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_cursor_item_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_cursor_item_clientbound_framed_dispatch.answer.jsonl",
        "play_set_cursor_item_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_cursor_item",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetCursorItemPacket")
    );
    assert_eq!(oracle.answer.input_item_empty, Some(true));
    assert_eq!(
        oracle.answer.input_item_empty,
        oracle.answer.stream_decoded_item_empty
    );
    assert_eq!(oracle.answer.input_item_empty, oracle.answer.decoded_item_empty);
    assert_eq!(body, vec![0]);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_cursor_item")
    .expect("dispatch Play clientbound set_cursor_item");

    match decoded {
        packet::Packet::PlaySetCursorItemClientbound(cursor_item) => {
            assert!(cursor_item.item.is_none());
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound set_cursor_item identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());

    let mut non_empty = [1u8].as_slice();
    let err = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut non_empty,
    )
    .expect_err("non-empty set_cursor_item fixture should be rejected before item registry bytes");
    assert!(
        err.to_string().contains("unsupported non-empty Play set_cursor_item ItemStack count 1"),
        "unexpected rejection: {err}"
    );
}
