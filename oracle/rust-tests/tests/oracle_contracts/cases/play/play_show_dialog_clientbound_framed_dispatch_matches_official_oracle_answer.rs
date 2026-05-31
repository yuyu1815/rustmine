#[test]
fn play_show_dialog_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_show_dialog_clientbound_framed_dispatch.test-manifest.json",
        "play_show_dialog_clientbound_framed_dispatch",
        "oracle/contracts/775/play_show_dialog_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_show_dialog_clientbound_framed_dispatch.answer.jsonl",
        "play_show_dialog_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:show_dialog",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundShowDialogPacket")
    );
    assert_eq!(
        oracle.answer.input_fixture.as_deref(),
        Some("Holder.direct(new NoticeDialog(CommonDialogData literal title, NoticeDialog.DEFAULT_ACTION))")
    );
    assert_eq!(
        oracle.answer.decoded_dialog_class.as_deref(),
        Some("net.minecraft.server.dialog.NoticeDialog")
    );
    assert_eq!(oracle.answer.input_dialog_title.as_deref(), Some("Oracle play notice"));
    assert_eq!(oracle.answer.input_dialog_title, oracle.answer.decoded_dialog_title);
    assert_eq!(oracle.answer.input_dialog_body_count, Some(0));
    assert_eq!(oracle.answer.input_dialog_input_count, Some(0));
    assert_eq!(oracle.answer.input_can_close_with_escape, Some(true));
    assert_eq!(oracle.answer.input_pause, Some(false));
    assert_eq!(oracle.answer.input_after_action.as_deref(), Some("close"));
    assert!(!body.is_empty());

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound show_dialog")
    .expect("dispatch Play clientbound show_dialog");

    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(packet.channel, "ShowDialog");
            assert_eq!(packet.data, body);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound show_dialog compatibility identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}
