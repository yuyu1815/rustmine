#[test]
fn play_custom_report_details_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_custom_report_details_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_custom_report_details_clientbound_framed_dispatch_body)
        .expect("spawn play_custom_report_details_clientbound oracle stack")
        .join()
        .expect("play_custom_report_details_clientbound oracle thread panicked");
}

fn play_custom_report_details_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_custom_report_details_clientbound_framed_dispatch.test-manifest.json",
        "play_custom_report_details_clientbound_framed_dispatch",
        "oracle/contracts/775/play_custom_report_details_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_custom_report_details_clientbound_framed_dispatch.answer.jsonl",
        "play_custom_report_details_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:custom_report_details",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundCustomReportDetailsPacket")
    );
    assert_eq!(
        oracle.answer.input_fixture.as_deref(),
        Some("Map.of() details")
    );
    assert_eq!(oracle.answer.input_detail_count, Some(0));
    assert_eq!(
        oracle.answer.input_detail_count,
        oracle.answer.stream_decoded_detail_count
    );
    assert_eq!(
        oracle.answer.input_detail_count,
        oracle.answer.decoded_detail_count
    );
    assert_eq!(
        oracle.answer.input_details,
        oracle.answer.stream_decoded_details
    );
    assert_eq!(oracle.answer.input_details, oracle.answer.decoded_details);
    assert_eq!(
        body,
        encode_varint(0),
        "official empty Play custom_report_details fixture should encode a zero-length details map"
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound custom_report_details")
    .expect("dispatch Play clientbound custom_report_details");

    match decoded {
        packet::Packet::PlayCustomReportDetailsClientbound(custom_report_details) => {
            assert_eq!(
                custom_report_details.detail_count.0, 0,
                "empty-map fixture must not decode report detail entries"
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound custom_report_details identity: {other:?}"
            )
        }
    }
    assert!(body_slice.is_empty());
}
