#[test]
fn play_update_tags_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_update_tags_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_update_tags_clientbound_framed_dispatch_body)
        .expect("spawn play_update_tags_clientbound oracle stack")
        .join()
        .expect("play_update_tags_clientbound oracle thread panicked");
}

fn play_update_tags_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_update_tags_clientbound_framed_dispatch.test-manifest.json",
        "play_update_tags_clientbound_framed_dispatch",
        "oracle/contracts/775/play_update_tags_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_update_tags_clientbound_framed_dispatch.answer.jsonl",
        "play_update_tags_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:update_tags",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundUpdateTagsPacket")
    );
    assert_eq!(oracle.answer.input_registry_payload_count, Some(0));
    assert_eq!(
        oracle.answer.input_registry_payload_count,
        oracle.answer.stream_decoded_registry_payload_count
    );
    assert_eq!(
        oracle.answer.input_registry_payload_count,
        oracle.answer.decoded_registry_payload_count
    );
    assert_eq!(
        body,
        encode_varint(0),
        "official empty Play update_tags fixture should encode a zero-length registry payload map"
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound update_tags")
    .expect("dispatch Play clientbound update_tags");

    match decoded {
        packet::Packet::PlayUpdateTagsClientbound(update_tags) => {
            assert_eq!(update_tags.registry_payload_count.0, 0);
        }
        other => panic!("decoded packet did not preserve Play clientbound update_tags identity: {other:?}"),
    }
    assert!(body_slice.is_empty());
}
