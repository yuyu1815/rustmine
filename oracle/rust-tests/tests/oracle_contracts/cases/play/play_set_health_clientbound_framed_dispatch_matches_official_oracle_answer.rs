#[test]
fn play_set_health_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_set_health_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_set_health_clientbound_framed_dispatch_body)
        .expect("spawn play_set_health_clientbound oracle stack")
        .join()
        .expect("play_set_health_clientbound oracle thread panicked");
}

fn play_set_health_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_health_clientbound_framed_dispatch.test-manifest.json",
        "play_set_health_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_health_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_health_clientbound_framed_dispatch.answer.jsonl",
        "play_set_health_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_health",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetHealthPacket")
    );
    let official_health = f32::from_be_bytes([body[0], body[1], body[2], body[3]]);
    let (official_food, food_len) = read_varint_prefix(&body[4..]);
    let saturation_offset = 4 + food_len;
    let official_saturation = f32::from_be_bytes([
        body[saturation_offset],
        body[saturation_offset + 1],
        body[saturation_offset + 2],
        body[saturation_offset + 3],
    ]);
    assert_eq!(saturation_offset + 4, body.len());

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_health")
    .expect("dispatch Play clientbound set_health");

    match decoded {
        packet::Packet::UpdateHealth(health) => {
            assert_eq!(health.health, official_health);
            assert_eq!(health.food.0, official_food);
            assert_eq!(health.food_saturation, official_saturation);
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound set_health identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}
