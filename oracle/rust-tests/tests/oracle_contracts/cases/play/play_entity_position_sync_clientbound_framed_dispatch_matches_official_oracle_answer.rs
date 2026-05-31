#[test]
fn play_entity_position_sync_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_entity_position_sync_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_entity_position_sync_clientbound_framed_dispatch_body)
        .expect("spawn play_entity_position_sync_clientbound oracle stack")
        .join()
        .expect("play_entity_position_sync_clientbound oracle thread panicked");
}

fn play_entity_position_sync_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_entity_position_sync_clientbound_framed_dispatch.test-manifest.json",
        "play_entity_position_sync_clientbound_framed_dispatch",
        "oracle/contracts/775/play_entity_position_sync_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_entity_position_sync_clientbound_framed_dispatch.answer.jsonl",
        "play_entity_position_sync_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:entity_position_sync",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket")
    );
    assert_eq!(
        oracle.answer.input_entity_id,
        oracle.answer.decoded_entity_id
    );
    assert_eq!(
        oracle.answer.stream_decoded_entity_id,
        oracle.answer.decoded_entity_id
    );
    assert_eq!(oracle.answer.input_x, oracle.answer.decoded_x);
    assert_eq!(oracle.answer.stream_decoded_x, oracle.answer.decoded_x);
    assert_eq!(oracle.answer.input_y, oracle.answer.decoded_y);
    assert_eq!(oracle.answer.stream_decoded_y, oracle.answer.decoded_y);
    assert_eq!(oracle.answer.input_z, oracle.answer.decoded_z);
    assert_eq!(oracle.answer.stream_decoded_z, oracle.answer.decoded_z);
    assert_eq!(oracle.answer.input_delta_x, oracle.answer.decoded_delta_x);
    assert_eq!(
        oracle.answer.stream_decoded_delta_x,
        oracle.answer.decoded_delta_x
    );
    assert_eq!(oracle.answer.input_delta_y, oracle.answer.decoded_delta_y);
    assert_eq!(
        oracle.answer.stream_decoded_delta_y,
        oracle.answer.decoded_delta_y
    );
    assert_eq!(oracle.answer.input_delta_z, oracle.answer.decoded_delta_z);
    assert_eq!(
        oracle.answer.stream_decoded_delta_z,
        oracle.answer.decoded_delta_z
    );
    assert_eq!(oracle.answer.input_y_rot, oracle.answer.decoded_y_rot);
    assert_eq!(
        oracle.answer.stream_decoded_y_rot,
        oracle.answer.decoded_y_rot
    );
    assert_eq!(oracle.answer.input_x_rot, oracle.answer.decoded_x_rot);
    assert_eq!(
        oracle.answer.stream_decoded_x_rot,
        oracle.answer.decoded_x_rot
    );
    assert_eq!(
        oracle.answer.input_on_ground,
        oracle.answer.decoded_on_ground
    );
    assert_eq!(
        oracle.answer.stream_decoded_on_ground,
        oracle.answer.decoded_on_ground
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound entity_position_sync")
    .expect("dispatch Play clientbound entity_position_sync");

    match decoded {
        packet::Packet::PlayEntityPositionSyncClientbound(sync) => {
            assert_eq!(sync.entity_id.0, oracle.answer.decoded_entity_id.unwrap());
            assert_eq!(sync.x, oracle.answer.decoded_x.unwrap());
            assert_eq!(sync.y, oracle.answer.decoded_y.unwrap());
            assert_eq!(sync.z, oracle.answer.decoded_z.unwrap());
            assert_eq!(sync.delta_x, oracle.answer.decoded_delta_x.unwrap());
            assert_eq!(sync.delta_y, oracle.answer.decoded_delta_y.unwrap());
            assert_eq!(sync.delta_z, oracle.answer.decoded_delta_z.unwrap());
            assert_eq!(sync.y_rot, oracle.answer.decoded_y_rot.unwrap());
            assert_eq!(sync.x_rot, oracle.answer.decoded_x_rot.unwrap());
            assert_eq!(sync.on_ground, oracle.answer.decoded_on_ground.unwrap());
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound entity_position_sync identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}

