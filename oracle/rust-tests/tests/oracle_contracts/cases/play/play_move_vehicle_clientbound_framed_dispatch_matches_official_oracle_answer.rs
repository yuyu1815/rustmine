#[test]
fn play_move_vehicle_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_move_vehicle_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_move_vehicle_clientbound_framed_dispatch_body)
        .expect("spawn play_move_vehicle_clientbound oracle stack")
        .join()
        .expect("play_move_vehicle_clientbound oracle thread panicked");
}

fn play_move_vehicle_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_move_vehicle_clientbound_framed_dispatch.test-manifest.json",
        "play_move_vehicle_clientbound_framed_dispatch",
        "oracle/contracts/775/play_move_vehicle_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_move_vehicle_clientbound_framed_dispatch.answer.jsonl",
        "play_move_vehicle_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:move_vehicle",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundMoveVehiclePacket")
    );
    assert_eq!(oracle.answer.input_x, oracle.answer.decoded_x);
    assert_eq!(oracle.answer.stream_decoded_x, oracle.answer.decoded_x);
    assert_eq!(oracle.answer.input_y, oracle.answer.decoded_y);
    assert_eq!(oracle.answer.stream_decoded_y, oracle.answer.decoded_y);
    assert_eq!(oracle.answer.input_z, oracle.answer.decoded_z);
    assert_eq!(oracle.answer.stream_decoded_z, oracle.answer.decoded_z);
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

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound move_vehicle")
    .expect("dispatch Play clientbound move_vehicle");

    match decoded {
        packet::Packet::VehicleTeleport(vehicle) => {
            assert_eq!(vehicle.x, oracle.answer.decoded_x.unwrap());
            assert_eq!(vehicle.y, oracle.answer.decoded_y.unwrap());
            assert_eq!(vehicle.z, oracle.answer.decoded_z.unwrap());
            assert_eq!(vehicle.yaw, oracle.answer.decoded_y_rot.unwrap());
            assert_eq!(vehicle.pitch, oracle.answer.decoded_x_rot.unwrap());
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound move_vehicle identity: {other:?}"
            )
        }
    }
    assert!(body_slice.is_empty());
}

