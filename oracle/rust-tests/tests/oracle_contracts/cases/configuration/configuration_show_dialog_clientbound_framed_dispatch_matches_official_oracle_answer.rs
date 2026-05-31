#[test]
fn configuration_show_dialog_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/configuration_show_dialog_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(
        manifest.case_id,
        "configuration_show_dialog_clientbound_framed_dispatch"
    );
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/configuration_show_dialog_clientbound_framed_dispatch.contract.json"
    );
    assert_eq!(
        manifest.answer_path,
        "oracle/answers/775/configuration_show_dialog_clientbound_framed_dispatch.answer.jsonl"
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "configuration_show_dialog_clientbound_framed_dispatch_matches_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/configuration_show_dialog_clientbound_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:show_dialog")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:show_dialog")
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
        oracle.answer.input_dialog_class.as_deref(),
        Some("net.minecraft.server.dialog.NoticeDialog")
    );
    assert_eq!(
        oracle.answer.decoded_dialog_class.as_deref(),
        Some("net.minecraft.server.dialog.NoticeDialog")
    );
    assert_eq!(
        oracle.answer.input_dialog_title, oracle.answer.decoded_dialog_title,
        "official decoded show_dialog title differs from the official input title"
    );
    assert_eq!(
        oracle.answer.input_dialog_body_count, oracle.answer.decoded_dialog_body_count,
        "official decoded show_dialog body count differs from the official input body count"
    );
    assert_eq!(
        oracle.answer.input_dialog_input_count, oracle.answer.decoded_dialog_input_count,
        "official decoded show_dialog input count differs from the official input count"
    );
    assert_eq!(
        oracle.answer.input_can_close_with_escape, oracle.answer.decoded_can_close_with_escape,
        "official decoded show_dialog escape-close flag differs from the official input flag"
    );
    assert_eq!(
        oracle.answer.input_pause, oracle.answer.decoded_pause,
        "official decoded show_dialog pause flag differs from the official input flag"
    );
    assert_eq!(
        oracle.answer.input_after_action, oracle.answer.decoded_after_action,
        "official decoded show_dialog after_action differs from the official input action"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:show_dialog",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("show_dialog answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert!(
        !body.is_empty(),
        "official show_dialog NoticeDialog fixture should encode a context-free dialog body"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Configuration,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Configuration clientbound show_dialog packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound show_dialog packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound show_dialog packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "ShowDialog",
                "decoded packet did not preserve show_dialog compatibility channel"
            );
            assert_eq!(
                packet.data,
                body,
                "decoded show_dialog compatibility packet did not retain the official context-free dialog body"
            );
        }
        other => {
            panic!("decoded packet did not preserve show_dialog identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound show_dialog packet did not consume the official body"
    );
}

