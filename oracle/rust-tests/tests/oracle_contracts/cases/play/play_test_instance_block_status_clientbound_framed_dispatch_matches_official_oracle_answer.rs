#[test]
fn play_test_instance_block_status_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_test_instance_block_status_clientbound_framed_dispatch.test-manifest.json",
        "play_test_instance_block_status_clientbound_framed_dispatch",
        "oracle/contracts/775/play_test_instance_block_status_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_test_instance_block_status_clientbound_framed_dispatch.answer.jsonl",
        "play_test_instance_block_status_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:test_instance_block_status",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundTestInstanceBlockStatus")
    );
    assert_eq!(
        oracle.answer.component_fixture.as_deref(),
        Some("Component.literal(\"rustmine test status\")")
    );
    assert_eq!(
        oracle.answer.input_component_text,
        oracle.answer.stream_decoded_component_text
    );
    assert_eq!(
        oracle.answer.input_component_text,
        oracle.answer.decoded_component_text
    );
    assert_eq!(oracle.answer.input_size_present, Some(false));
    assert_eq!(oracle.answer.input_size_present, oracle.answer.stream_decoded_size_present);
    assert_eq!(oracle.answer.input_size_present, oracle.answer.decoded_size_present);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound test_instance_block_status")
    .expect("dispatch Play clientbound test_instance_block_status");

    match decoded {
        packet::Packet::PlayTestInstanceBlockStatusClientbound(status) => {
            assert_eq!(
                status.status.to_string(),
                oracle.answer.decoded_component_text.unwrap_or_default()
            );
            assert!(!status.size_present);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound test_instance_block_status identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());

    let mut unsupported_body = body.clone();
    *unsupported_body.last_mut().expect("optional marker") = 1;
    let err = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut unsupported_body.as_slice(),
    )
    .expect_err("present test_instance_block_status size must stay unsupported");
    assert!(
        err.to_string()
            .contains("unsupported Play test_instance_block_status present size"),
        "unexpected error: {err}"
    );
}
