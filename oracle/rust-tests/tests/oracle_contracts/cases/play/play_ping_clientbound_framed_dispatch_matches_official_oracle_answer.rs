#[test]
fn play_ping_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_ping_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_ping_clientbound_framed_dispatch_body)
        .expect("spawn play_ping_clientbound oracle stack")
        .join()
        .expect("play_ping_clientbound oracle thread panicked");
}

fn play_ping_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_ping_clientbound_framed_dispatch.test-manifest.json",
        "play_ping_clientbound_framed_dispatch",
        "oracle/contracts/775/play_ping_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_ping_clientbound_framed_dispatch.answer.jsonl",
        "play_ping_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:ping",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundPingPacket")
    );
    assert_eq!(oracle.answer.decoded_id, Some(oracle.answer.input_id));
    assert_eq!(
        oracle.answer.stream_decoded_id,
        Some(oracle.answer.input_id)
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound ping")
    .expect("dispatch Play clientbound ping");

    match decoded {
        packet::Packet::PlayPingClientbound_i32(ping) => {
            assert_eq!(i64::from(ping.id), oracle.answer.input_id);
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound ping identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}

