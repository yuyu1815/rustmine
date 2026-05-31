#[test]
fn play_forget_level_chunk_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_forget_level_chunk_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_forget_level_chunk_clientbound_framed_dispatch_body)
        .expect("spawn play_forget_level_chunk_clientbound oracle stack")
        .join()
        .expect("play_forget_level_chunk_clientbound oracle thread panicked");
}

fn play_forget_level_chunk_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_forget_level_chunk_clientbound_framed_dispatch.test-manifest.json",
        "play_forget_level_chunk_clientbound_framed_dispatch",
        "oracle/contracts/775/play_forget_level_chunk_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_forget_level_chunk_clientbound_framed_dispatch.answer.jsonl",
        "play_forget_level_chunk_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:forget_level_chunk",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundForgetLevelChunkPacket")
    );
    assert_eq!(oracle.answer.input_chunk_x, oracle.answer.decoded_chunk_x);
    assert_eq!(
        oracle.answer.stream_decoded_chunk_x,
        oracle.answer.decoded_chunk_x
    );
    assert_eq!(oracle.answer.input_chunk_z, oracle.answer.decoded_chunk_z);
    assert_eq!(
        oracle.answer.stream_decoded_chunk_z,
        oracle.answer.decoded_chunk_z
    );
    assert_eq!(
        oracle.answer.input_chunk_pos_packed,
        oracle.answer.decoded_chunk_pos_packed
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound forget_level_chunk")
    .expect("dispatch Play clientbound forget_level_chunk");

    match decoded {
        packet::Packet::PlayForgetLevelChunkClientbound(forget) => {
            assert_eq!(
                forget.chunk_pos,
                oracle.answer.decoded_chunk_pos_packed.unwrap()
            );
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound forget_level_chunk identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}

