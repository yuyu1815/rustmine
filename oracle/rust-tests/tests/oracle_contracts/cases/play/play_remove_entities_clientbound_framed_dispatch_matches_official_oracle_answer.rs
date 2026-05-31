#[test]
fn play_remove_entities_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_remove_entities_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_remove_entities_clientbound_framed_dispatch_body)
        .expect("spawn play_remove_entities_clientbound oracle stack")
        .join()
        .expect("play_remove_entities_clientbound oracle thread panicked");
}

fn play_remove_entities_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_remove_entities_clientbound_framed_dispatch.test-manifest.json",
        "play_remove_entities_clientbound_framed_dispatch",
        "oracle/contracts/775/play_remove_entities_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_remove_entities_clientbound_framed_dispatch.answer.jsonl",
        "play_remove_entities_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:remove_entities",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundRemoveEntitiesPacket")
    );
    assert_eq!(
        oracle.answer.decoded_entity_ids,
        oracle.answer.input_entity_ids
    );
    assert_eq!(
        oracle.answer.stream_decoded_entity_ids,
        oracle.answer.input_entity_ids
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound remove_entities")
    .expect("dispatch Play clientbound remove_entities");

    match decoded {
        packet::Packet::EntityDestroy(remove) => {
            let decoded_ids: Vec<i32> = remove.entity_ids.data.iter().map(|id| id.0).collect();
            assert_eq!(
                decoded_ids,
                oracle.answer.decoded_entity_ids.clone().unwrap()
            );
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound remove_entities identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}

