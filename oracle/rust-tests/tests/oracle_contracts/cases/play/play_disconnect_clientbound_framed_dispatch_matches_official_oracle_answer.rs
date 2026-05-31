#[test]
fn play_disconnect_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_disconnect_clientbound_framed_dispatch.test-manifest.json",
        "play_disconnect_clientbound_framed_dispatch",
        "oracle/contracts/775/play_disconnect_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_disconnect_clientbound_framed_dispatch.answer.jsonl",
        "play_disconnect_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:disconnect",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundDisconnectPacket")
    );
    assert_eq!(
        oracle.answer.reason_fixture.as_deref(),
        Some("Component.literal(\"\")")
    );
    assert_eq!(
        oracle.answer.input_reason_text,
        oracle.answer.stream_decoded_reason_text
    );
    assert_eq!(
        oracle.answer.input_reason_text,
        oracle.answer.decoded_reason_text
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound disconnect")
    .expect("dispatch Play clientbound disconnect");

    match decoded {
        packet::Packet::Disconnect(disconnect) => {
            assert_eq!(
                disconnect.reason.to_string(),
                oracle.answer.decoded_reason_text.unwrap_or_default()
            );
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound disconnect identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}

