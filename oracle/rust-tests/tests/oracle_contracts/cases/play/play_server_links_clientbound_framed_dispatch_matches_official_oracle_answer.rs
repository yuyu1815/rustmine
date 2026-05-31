#[test]
fn play_server_links_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_server_links_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_server_links_clientbound_framed_dispatch_body)
        .expect("spawn play_server_links_clientbound oracle stack")
        .join()
        .expect("play_server_links_clientbound oracle thread panicked");
}

fn play_server_links_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_server_links_clientbound_framed_dispatch.test-manifest.json",
        "play_server_links_clientbound_framed_dispatch",
        "oracle/contracts/775/play_server_links_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_server_links_clientbound_framed_dispatch.answer.jsonl",
        "play_server_links_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:server_links",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundServerLinksPacket")
    );
    assert_eq!(
        oracle.answer.input_fixture.as_deref(),
        Some("List.of() server_links")
    );
    assert_eq!(oracle.answer.input_link_count, Some(0));
    assert_eq!(
        oracle.answer.input_link_count,
        oracle.answer.stream_decoded_link_count
    );
    assert_eq!(oracle.answer.input_link_count, oracle.answer.decoded_link_count);
    assert_eq!(oracle.answer.input_links, oracle.answer.stream_decoded_links);
    assert_eq!(oracle.answer.input_links, oracle.answer.decoded_links);
    assert_eq!(
        body,
        encode_varint(0),
        "official empty Play server_links fixture should encode a zero-length links list"
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound server_links")
    .expect("dispatch Play clientbound server_links");

    match decoded {
        packet::Packet::PlayServerLinksClientbound(server_links) => {
            assert_eq!(
                server_links.link_count.0, 0,
                "empty-list fixture must not decode server link entries"
            );
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound server_links identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}
