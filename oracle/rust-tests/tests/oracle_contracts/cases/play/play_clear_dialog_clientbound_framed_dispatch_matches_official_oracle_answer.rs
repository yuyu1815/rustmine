#[test]
fn play_clear_dialog_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_clear_dialog_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_clear_dialog_clientbound_framed_dispatch_body)
        .expect("spawn play_clear_dialog_clientbound oracle stack")
        .join()
        .expect("play_clear_dialog_clientbound oracle thread panicked");
}

fn play_clear_dialog_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_clear_dialog_clientbound_framed_dispatch.test-manifest.json",
        "play_clear_dialog_clientbound_framed_dispatch",
        "oracle/contracts/775/play_clear_dialog_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_clear_dialog_clientbound_framed_dispatch.answer.jsonl",
        "play_clear_dialog_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:clear_dialog",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundClearDialogPacket")
    );
    assert_eq!(
        oracle.answer.input_fixture.as_deref(),
        Some("ClientboundClearDialogPacket.INSTANCE")
    );
    assert_eq!(oracle.answer.stream_decoded_same_instance, Some(true));
    assert_eq!(oracle.answer.decoded_equals_instance, Some(true));
    assert!(
        body.is_empty(),
        "official Play clear_dialog singleton body should be empty"
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound clear_dialog")
    .expect("dispatch Play clientbound clear_dialog");

    match decoded {
        packet::Packet::PlayClearDialogClientbound(clear_dialog) => {
            assert_eq!(clear_dialog.empty, ());
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound clear_dialog identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}
