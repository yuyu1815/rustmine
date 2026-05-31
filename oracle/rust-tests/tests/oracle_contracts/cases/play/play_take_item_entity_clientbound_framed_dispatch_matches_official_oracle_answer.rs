#[test]
fn play_take_item_entity_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_take_item_entity_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_take_item_entity_clientbound_framed_dispatch_body)
        .expect("spawn play_take_item_entity_clientbound oracle stack")
        .join()
        .expect("play_take_item_entity_clientbound oracle thread panicked");
}

fn play_take_item_entity_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_take_item_entity_clientbound_framed_dispatch.test-manifest.json",
        "play_take_item_entity_clientbound_framed_dispatch",
        "oracle/contracts/775/play_take_item_entity_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_take_item_entity_clientbound_framed_dispatch.answer.jsonl",
        "play_take_item_entity_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:take_item_entity",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundTakeItemEntityPacket")
    );
    assert_eq!(oracle.answer.stream_decoded_item_id, oracle.answer.decoded_item_id);
    assert_eq!(oracle.answer.stream_decoded_player_id, oracle.answer.decoded_player_id);
    assert_eq!(oracle.answer.stream_decoded_amount, oracle.answer.decoded_amount);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound take_item_entity")
    .expect("dispatch Play clientbound take_item_entity");

    match decoded {
        packet::Packet::CollectItem(collect_item) => {
            assert_eq!(
                collect_item.collected_entity_id.0,
                oracle.answer.decoded_item_id.unwrap()
            );
            assert_eq!(
                collect_item.collector_entity_id.0,
                oracle.answer.decoded_player_id.unwrap()
            );
            assert_eq!(
                collect_item.number_of_items.0,
                oracle.answer.decoded_amount.unwrap()
            );
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound take_item_entity identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}
