#[test]
fn play_set_objective_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_objective_clientbound_framed_dispatch.test-manifest.json",
        "play_set_objective_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_objective_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_objective_clientbound_framed_dispatch.answer.jsonl",
        "play_set_objective_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_objective",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetObjectivePacket")
    );
    assert_eq!(
        oracle.answer.input_objective_name.as_deref(),
        Some("rustmine_objective")
    );
    assert_eq!(
        oracle.answer.input_objective_name,
        oracle.answer.stream_decoded_objective_name
    );
    assert_eq!(oracle.answer.input_objective_name, oracle.answer.decoded_objective_name);
    assert_eq!(oracle.answer.input_method, Some(1));
    assert_eq!(oracle.answer.input_method, oracle.answer.stream_decoded_method);
    assert_eq!(oracle.answer.input_method, oracle.answer.decoded_method);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_objective")
    .expect("dispatch Play clientbound set_objective");

    match decoded {
        packet::Packet::PlaySetObjectiveClientbound(objective) => {
            assert_eq!(objective.objective_name, "rustmine_objective");
            assert_eq!(objective.method, 1);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound set_objective identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());

    let mut unsupported_body = body.clone();
    *unsupported_body.last_mut().expect("method byte") = 0;
    let err = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut unsupported_body.as_slice(),
    )
    .expect_err("add/change set_objective branches must stay unsupported");
    assert!(
        err.to_string()
            .contains("unsupported Play set_objective method 0"),
        "unexpected error: {err}"
    );
}
