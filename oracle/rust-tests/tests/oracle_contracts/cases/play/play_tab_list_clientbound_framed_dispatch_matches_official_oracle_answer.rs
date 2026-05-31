#[test]
fn play_tab_list_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_tab_list_clientbound_framed_dispatch.test-manifest.json",
        "play_tab_list_clientbound_framed_dispatch",
        "oracle/contracts/775/play_tab_list_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_tab_list_clientbound_framed_dispatch.answer.jsonl",
        "play_tab_list_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:tab_list",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundTabListPacket")
    );
    assert_eq!(
        oracle.answer.header_component_fixture.as_deref(),
        Some("Component.literal(\"rustmine tab header\")")
    );
    assert_eq!(
        oracle.answer.footer_component_fixture.as_deref(),
        Some("Component.literal(\"rustmine tab footer\")")
    );
    assert_eq!(
        oracle.answer.input_header_text,
        oracle.answer.stream_decoded_header_text
    );
    assert_eq!(oracle.answer.input_header_text, oracle.answer.decoded_header_text);
    assert_eq!(
        oracle.answer.input_footer_text,
        oracle.answer.stream_decoded_footer_text
    );
    assert_eq!(oracle.answer.input_footer_text, oracle.answer.decoded_footer_text);

    let mut expected = vec![8, 0, "rustmine tab header".len() as u8];
    expected.extend_from_slice(b"rustmine tab header");
    expected.extend_from_slice(&[8, 0, "rustmine tab footer".len() as u8]);
    expected.extend_from_slice(b"rustmine tab footer");
    assert_eq!(body, expected);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound tab_list")
    .expect("dispatch Play clientbound tab_list");

    match decoded {
        packet::Packet::PlayTabListClientbound(tab_list) => {
            assert_eq!(
                tab_list.header.to_string(),
                oracle.answer.decoded_header_text.unwrap_or_default()
            );
            assert_eq!(
                tab_list.footer.to_string(),
                oracle.answer.decoded_footer_text.unwrap_or_default()
            );
        }
        other => panic!("decoded packet did not preserve Play clientbound tab_list identity: {other:?}"),
    }
    assert!(body_slice.is_empty());
}
