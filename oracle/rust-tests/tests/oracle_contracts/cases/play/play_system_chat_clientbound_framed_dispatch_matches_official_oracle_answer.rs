#[test]
fn play_system_chat_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_system_chat_clientbound_framed_dispatch.test-manifest.json",
        "play_system_chat_clientbound_framed_dispatch",
        "oracle/contracts/775/play_system_chat_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_system_chat_clientbound_framed_dispatch.answer.jsonl",
        "play_system_chat_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:system_chat",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSystemChatPacket")
    );
    assert_eq!(
        oracle.answer.component_fixture.as_deref(),
        Some("Component.literal(\"rustmine system chat\")")
    );
    assert_eq!(
        oracle.answer.input_component_text,
        oracle.answer.stream_decoded_component_text
    );
    assert_eq!(
        oracle.answer.input_component_text,
        oracle.answer.decoded_component_text
    );
    assert_eq!(oracle.answer.input_overlay, Some(false));
    assert_eq!(oracle.answer.input_overlay, oracle.answer.stream_decoded_overlay);
    assert_eq!(oracle.answer.input_overlay, oracle.answer.decoded_overlay);

    let mut expected = vec![8, 0, "rustmine system chat".len() as u8];
    expected.extend_from_slice(b"rustmine system chat");
    expected.push(0);
    assert_eq!(body, expected);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound system_chat")
    .expect("dispatch Play clientbound system_chat");

    match decoded {
        packet::Packet::PlaySystemChatClientbound(system_chat) => {
            assert_eq!(
                system_chat.content.to_string(),
                oracle.answer.decoded_component_text.unwrap_or_default()
            );
            assert!(!system_chat.overlay);
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound system_chat identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}
