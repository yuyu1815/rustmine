#[test]
fn play_set_default_spawn_position_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_default_spawn_position_clientbound_framed_dispatch.test-manifest.json",
        "play_set_default_spawn_position_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_default_spawn_position_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_default_spawn_position_clientbound_framed_dispatch.answer.jsonl",
        "play_set_default_spawn_position_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_default_spawn_position",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetDefaultSpawnPositionPacket")
    );
    assert_eq!(oracle.answer.input_dimension.as_deref(), Some("minecraft:overworld"));
    assert_eq!(oracle.answer.input_dimension, oracle.answer.stream_decoded_dimension);
    assert_eq!(oracle.answer.input_dimension, oracle.answer.decoded_dimension);
    assert_eq!(oracle.answer.input_block_x, Some(0));
    assert_eq!(oracle.answer.input_block_y, Some(0));
    assert_eq!(oracle.answer.input_block_z, Some(0));
    assert_eq!(oracle.answer.input_yaw, Some(0.0));
    assert_eq!(oracle.answer.input_pitch, Some(0.0));
    assert_eq!(
        body,
        decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex")
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_default_spawn_position")
    .expect("dispatch Play clientbound set_default_spawn_position");

    match decoded {
        packet::Packet::PlaySetDefaultSpawnPositionClientbound(spawn) => {
            assert_eq!(spawn.dimension, "minecraft:overworld");
            assert_eq!(spawn.location.x, 0);
            assert_eq!(spawn.location.y, 0);
            assert_eq!(spawn.location.z, 0);
            assert_eq!(spawn.yaw, 0.0);
            assert_eq!(spawn.pitch, 0.0);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound set_default_spawn_position identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}
