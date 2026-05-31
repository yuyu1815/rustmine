#[test]
fn play_mount_screen_open_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_mount_screen_open_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_mount_screen_open_clientbound_framed_dispatch_body)
        .expect("spawn play_mount_screen_open_clientbound oracle stack")
        .join()
        .expect("play_mount_screen_open_clientbound oracle thread panicked");
}

fn play_mount_screen_open_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_mount_screen_open_clientbound_framed_dispatch.test-manifest.json",
        "play_mount_screen_open_clientbound_framed_dispatch",
        "oracle/contracts/775/play_mount_screen_open_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_mount_screen_open_clientbound_framed_dispatch.answer.jsonl",
        "play_mount_screen_open_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:mount_screen_open",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundMountScreenOpenPacket")
    );
    assert_eq!(
        oracle.answer.input_container_id,
        oracle.answer.decoded_container_id
    );
    assert_eq!(
        oracle.answer.input_inventory_columns,
        oracle.answer.decoded_inventory_columns
    );
    assert_eq!(
        oracle.answer.stream_decoded_inventory_columns,
        oracle.answer.decoded_inventory_columns
    );
    assert_eq!(
        oracle.answer.input_entity_id,
        oracle.answer.decoded_entity_id
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound mount_screen_open")
    .expect("dispatch Play clientbound mount_screen_open");

    match decoded {
        packet::Packet::PlayMountScreenOpenClientbound(mount) => {
            assert_eq!(
                mount.container_id.0,
                oracle.answer.decoded_container_id.unwrap()
            );
            assert_eq!(
                mount.inventory_columns.0,
                oracle.answer.decoded_inventory_columns.unwrap()
            );
            assert_eq!(mount.entity_id, oracle.answer.decoded_entity_id.unwrap());
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound mount_screen_open identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}

