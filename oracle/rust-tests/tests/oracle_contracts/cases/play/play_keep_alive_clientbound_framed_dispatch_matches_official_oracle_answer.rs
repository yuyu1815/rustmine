#[test]
fn play_keep_alive_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_keep_alive_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_keep_alive_clientbound_framed_dispatch_body)
        .expect("spawn play_keep_alive_clientbound oracle stack")
        .join()
        .expect("play_keep_alive_clientbound oracle thread panicked");
}

fn play_keep_alive_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_keep_alive_clientbound_framed_dispatch.test-manifest.json",
        "play_keep_alive_clientbound_framed_dispatch",
        "oracle/contracts/775/play_keep_alive_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_keep_alive_clientbound_framed_dispatch.answer.jsonl",
        "play_keep_alive_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:keep_alive",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundKeepAlivePacket")
    );
    assert_eq!(
        oracle.answer.input_id,
        oracle.answer.stream_decoded_id.unwrap()
    );
    assert_eq!(oracle.answer.input_id, oracle.answer.decoded_id.unwrap());

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound keep_alive")
    .expect("dispatch Play clientbound keep_alive");

    match decoded {
        packet::Packet::KeepAliveClientbound_i64(keep_alive) => {
            assert_eq!(keep_alive.id, oracle.answer.decoded_id.unwrap());
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound keep_alive identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}

