#[test]
fn play_move_entity_rot_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_move_entity_rot_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_move_entity_rot_clientbound_framed_dispatch_body)
        .expect("spawn play_move_entity_rot_clientbound oracle stack")
        .join()
        .expect("play_move_entity_rot_clientbound oracle thread panicked");
}

fn play_move_entity_rot_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_move_entity_rot_clientbound_framed_dispatch.test-manifest.json",
        "play_move_entity_rot_clientbound_framed_dispatch",
        "oracle/contracts/775/play_move_entity_rot_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_move_entity_rot_clientbound_framed_dispatch.answer.jsonl",
        "play_move_entity_rot_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:move_entity_rot",
    );
    assert_move_entity_common(
        &oracle,
        "net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Rot",
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound move_entity_rot")
    .expect("dispatch Play clientbound move_entity_rot");

    match decoded {
        packet::Packet::EntityLook_VarInt(rot) => {
            assert_eq!(rot.entity_id.0, oracle.answer.decoded_entity_id.unwrap());
            assert_eq!(
                i32::from(rot.yaw),
                oracle.answer.decoded_move_y_rot_byte.unwrap()
            );
            assert_eq!(
                i32::from(rot.pitch),
                oracle.answer.decoded_move_x_rot_byte.unwrap()
            );
            assert_eq!(rot.on_ground, oracle.answer.decoded_on_ground.unwrap());
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound move_entity_rot identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}

