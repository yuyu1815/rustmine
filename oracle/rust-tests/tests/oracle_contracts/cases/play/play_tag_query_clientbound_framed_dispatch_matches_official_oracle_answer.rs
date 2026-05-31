#[test]
fn play_tag_query_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_tag_query_clientbound_framed_dispatch.test-manifest.json",
        "play_tag_query_clientbound_framed_dispatch",
        "oracle/contracts/775/play_tag_query_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_tag_query_clientbound_framed_dispatch.answer.jsonl",
        "play_tag_query_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:tag_query",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundTagQueryPacket")
    );
    assert_eq!(oracle.answer.input_transaction_id, Some(123));
    assert_eq!(
        oracle.answer.input_transaction_id,
        oracle.answer.stream_decoded_transaction_id
    );
    assert_eq!(oracle.answer.input_transaction_id, oracle.answer.decoded_transaction_id);
    assert_eq!(oracle.answer.input_tag_size, Some(0));
    assert_eq!(oracle.answer.input_tag_snbt.as_deref(), Some("{}"));
    assert_eq!(body, vec![0x7b, 0x0a, 0x00]);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound tag_query")
    .expect("dispatch Play clientbound tag_query");

    match decoded {
        packet::Packet::PlayTagQueryClientbound(tag_query) => {
            assert_eq!(tag_query.transaction_id.0, 123);
            assert_eq!(tag_query.nbt_tag_type, 10);
            assert_eq!(tag_query.tag, vec![0]);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound tag_query identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());

    let mut unsupported_body: &[u8] = &[0x7b, 0x0a, 0x01];
    let err = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut unsupported_body,
    )
    .expect_err("non-empty tag_query compound must stay unsupported");
    assert!(
        err.to_string()
            .contains("unsupported non-empty Play tag_query compound payload"),
        "unexpected error: {err}"
    );
}
