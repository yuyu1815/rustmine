#[test]
fn play_set_entity_motion_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_set_entity_motion_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_set_entity_motion_clientbound_framed_dispatch_body)
        .expect("spawn play_set_entity_motion_clientbound oracle stack")
        .join()
        .expect("play_set_entity_motion_clientbound oracle thread panicked");
}

fn play_set_entity_motion_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_entity_motion_clientbound_framed_dispatch.test-manifest.json",
        "play_set_entity_motion_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_entity_motion_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_entity_motion_clientbound_framed_dispatch.answer.jsonl",
        "play_set_entity_motion_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_entity_motion",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetEntityMotionPacket")
    );
    assert_eq!(
        oracle.answer.stream_decoded_entity_id,
        oracle.answer.decoded_entity_id
    );

    let (official_entity_id, mut offset) = read_varint_prefix(&body);
    let official_velocity_x = i16::from_be_bytes([body[offset], body[offset + 1]]);
    offset += 2;
    let official_velocity_y = i16::from_be_bytes([body[offset], body[offset + 1]]);
    offset += 2;
    let official_velocity_z = i16::from_be_bytes([body[offset], body[offset + 1]]);
    offset += 2;
    assert_eq!(offset, body.len());

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_entity_motion")
    .expect("dispatch Play clientbound set_entity_motion");

    match decoded {
        packet::Packet::EntityVelocity(velocity) => {
            assert_eq!(velocity.entity_id.0, official_entity_id);
            assert_eq!(velocity.velocity_x, official_velocity_x);
            assert_eq!(velocity.velocity_y, official_velocity_y);
            assert_eq!(velocity.velocity_z, official_velocity_z);
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound set_entity_motion identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}
