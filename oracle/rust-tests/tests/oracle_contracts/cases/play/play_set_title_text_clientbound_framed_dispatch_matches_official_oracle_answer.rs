#[test]
fn play_set_title_text_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_title_text_clientbound_framed_dispatch.test-manifest.json",
        "play_set_title_text_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_title_text_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_title_text_clientbound_framed_dispatch.answer.jsonl",
        "play_set_title_text_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_title_text",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetTitleTextPacket")
    );
    assert_eq!(
        oracle.answer.component_fixture.as_deref(),
        Some("Component.literal(\"rustmine title\")")
    );
    assert_eq!(
        oracle.answer.input_component_text,
        oracle.answer.stream_decoded_component_text
    );
    assert_eq!(
        oracle.answer.input_component_text,
        oracle.answer.decoded_component_text
    );

    let mut expected = vec![8, 0, "rustmine title".len() as u8];
    expected.extend_from_slice(b"rustmine title");
    assert_eq!(body, expected);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_title_text")
    .expect("dispatch Play clientbound set_title_text");

    match decoded {
        packet::Packet::PlaySetTitleTextClientbound(title) => {
            assert_eq!(
                title.text.to_string(),
                oracle.answer.decoded_component_text.unwrap_or_default()
            );
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound set_title_text identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}
