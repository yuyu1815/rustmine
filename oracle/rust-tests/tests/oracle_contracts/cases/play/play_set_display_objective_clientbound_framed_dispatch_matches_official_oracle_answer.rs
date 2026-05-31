#[test]
fn play_set_display_objective_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_set_display_objective_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_set_display_objective_clientbound_framed_dispatch_body)
        .expect("spawn play_set_display_objective_clientbound oracle stack")
        .join()
        .expect("play_set_display_objective_clientbound oracle thread panicked");
}

fn play_set_display_objective_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_display_objective_clientbound_framed_dispatch.test-manifest.json",
        "play_set_display_objective_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_display_objective_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_display_objective_clientbound_framed_dispatch.answer.jsonl",
        "play_set_display_objective_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_display_objective",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetDisplayObjectivePacket")
    );
    assert_eq!(oracle.answer.input_display_slot.as_deref(), Some("LIST"));
    assert_eq!(oracle.answer.input_display_slot_id, Some(0));
    assert_eq!(
        oracle.answer.input_display_slot,
        oracle.answer.stream_decoded_display_slot
    );
    assert_eq!(
        oracle.answer.input_display_slot,
        oracle.answer.decoded_display_slot
    );
    assert_eq!(oracle.answer.input_objective_present, Some(false));
    assert_eq!(
        oracle.answer.input_objective_present,
        oracle.answer.stream_decoded_objective_present
    );
    assert_eq!(
        oracle.answer.input_objective_present,
        oracle.answer.decoded_objective_present
    );
    assert_eq!(
        body,
        vec![0, 0],
        "official clear-slot fixture should encode DisplaySlot.LIST and an empty objective string"
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_display_objective")
    .expect("dispatch Play clientbound set_display_objective");

    match decoded {
        packet::Packet::PlaySetDisplayObjectiveClientbound(display_objective) => {
            assert_eq!(display_objective.slot.0, 0);
            assert_eq!(display_objective.objective_name, "");
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound set_display_objective identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}
