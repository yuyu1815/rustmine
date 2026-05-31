#[test]
fn play_move_entity_pos_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_move_entity_pos_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_move_entity_pos_clientbound_framed_dispatch_body)
        .expect("spawn play_move_entity_pos_clientbound oracle stack")
        .join()
        .expect("play_move_entity_pos_clientbound oracle thread panicked");
}

fn play_move_entity_pos_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_move_entity_pos_clientbound_framed_dispatch.test-manifest.json",
        "play_move_entity_pos_clientbound_framed_dispatch",
        "oracle/contracts/775/play_move_entity_pos_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_move_entity_pos_clientbound_framed_dispatch.answer.jsonl",
        "play_move_entity_pos_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:move_entity_pos",
    );
    assert_move_entity_common(
        &oracle,
        "net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos",
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound move_entity_pos")
    .expect("dispatch Play clientbound move_entity_pos");

    match decoded {
        packet::Packet::EntityMove_i16(movement) => {
            assert_eq!(
                movement.entity_id.0,
                oracle.answer.decoded_entity_id.unwrap()
            );
            assert_eq!(
                f64::from(movement.delta_x),
                oracle.answer.decoded_xa.unwrap() as f64 / 4096.0
            );
            assert_eq!(
                f64::from(movement.delta_y),
                oracle.answer.decoded_ya.unwrap() as f64 / 4096.0
            );
            assert_eq!(
                f64::from(movement.delta_z),
                oracle.answer.decoded_za.unwrap() as f64 / 4096.0
            );
            assert_eq!(movement.on_ground, oracle.answer.decoded_on_ground.unwrap());
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound move_entity_pos identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}

