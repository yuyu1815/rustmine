#[test]
fn play_initialize_border_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_initialize_border_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_initialize_border_clientbound_framed_dispatch_body)
        .expect("spawn play_initialize_border_clientbound oracle stack")
        .join()
        .expect("play_initialize_border_clientbound oracle thread panicked");
}

fn play_initialize_border_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_initialize_border_clientbound_framed_dispatch.test-manifest.json",
        "play_initialize_border_clientbound_framed_dispatch",
        "oracle/contracts/775/play_initialize_border_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_initialize_border_clientbound_framed_dispatch.answer.jsonl",
        "play_initialize_border_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:initialize_border",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundInitializeBorderPacket")
    );
    assert_eq!(
        oracle.answer.input_new_center_x,
        oracle.answer.decoded_new_center_x
    );
    assert_eq!(
        oracle.answer.input_new_center_z,
        oracle.answer.decoded_new_center_z
    );
    assert_eq!(oracle.answer.input_old_size, oracle.answer.decoded_old_size);
    assert_eq!(oracle.answer.input_new_size, oracle.answer.decoded_new_size);
    assert_eq!(
        oracle.answer.input_lerp_time,
        oracle.answer.decoded_lerp_time
    );
    assert_eq!(
        oracle.answer.input_new_absolute_max_size,
        oracle.answer.decoded_new_absolute_max_size
    );
    assert_eq!(
        oracle.answer.input_warning_blocks,
        oracle.answer.decoded_warning_blocks
    );
    assert_eq!(
        oracle.answer.input_warning_time,
        oracle.answer.decoded_warning_time
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound initialize_border")
    .expect("dispatch Play clientbound initialize_border");

    match decoded {
        packet::Packet::PlayInitializeBorderClientbound(border) => {
            assert_eq!(
                border.new_center_x,
                oracle.answer.decoded_new_center_x.unwrap()
            );
            assert_eq!(
                border.new_center_z,
                oracle.answer.decoded_new_center_z.unwrap()
            );
            assert_eq!(border.old_size, oracle.answer.decoded_old_size.unwrap());
            assert_eq!(border.new_size, oracle.answer.decoded_new_size.unwrap());
            assert_eq!(border.lerp_time.0, oracle.answer.decoded_lerp_time.unwrap());
            assert_eq!(
                border.new_absolute_max_size.0,
                oracle.answer.decoded_new_absolute_max_size.unwrap()
            );
            assert_eq!(
                border.warning_blocks.0,
                oracle.answer.decoded_warning_blocks.unwrap()
            );
            assert_eq!(
                border.warning_time.0,
                oracle.answer.decoded_warning_time.unwrap()
            );
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound initialize_border identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}

