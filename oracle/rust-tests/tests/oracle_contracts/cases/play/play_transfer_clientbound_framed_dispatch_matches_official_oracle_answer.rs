#[test]
fn play_transfer_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_transfer_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_transfer_clientbound_framed_dispatch_body)
        .expect("spawn play_transfer_clientbound oracle stack")
        .join()
        .expect("play_transfer_clientbound oracle thread panicked");
}

fn play_transfer_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_transfer_clientbound_framed_dispatch.test-manifest.json",
        "play_transfer_clientbound_framed_dispatch",
        "oracle/contracts/775/play_transfer_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_transfer_clientbound_framed_dispatch.answer.jsonl",
        "play_transfer_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:transfer",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundTransferPacket")
    );
    assert_eq!(oracle.answer.input_host, oracle.answer.stream_decoded_host);
    assert_eq!(oracle.answer.input_host, oracle.answer.decoded_host);
    assert_eq!(oracle.answer.input_port, oracle.answer.stream_decoded_port);
    assert_eq!(oracle.answer.input_port, oracle.answer.decoded_port);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound transfer")
    .expect("dispatch Play clientbound transfer");

    match decoded {
        packet::Packet::PlayTransferClientbound(transfer) => {
            assert_eq!(
                Some(transfer.host),
                oracle.answer.decoded_host,
                "decoded Play transfer host differs from official answer"
            );
            assert_eq!(
                Some(transfer.port.0),
                oracle.answer.decoded_port,
                "decoded Play transfer port differs from official answer"
            );
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound transfer identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}
