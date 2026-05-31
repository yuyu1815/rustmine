#[test]
fn play_add_entity_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_add_entity_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_add_entity_clientbound_framed_dispatch_body)
        .expect("spawn play_add_entity_clientbound oracle stack")
        .join()
        .expect("play_add_entity_clientbound oracle thread panicked");
}

fn play_add_entity_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/play_add_entity_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(manifest.case_id, "play_add_entity_clientbound_framed_dispatch");
    assert_eq!(manifest.contract_path, "oracle/contracts/775/play_add_entity_clientbound_framed_dispatch.contract.json");
    assert_eq!(manifest.answer_path, "oracle/answers/775/play_add_entity_clientbound_framed_dispatch.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "play_add_entity_clientbound_framed_dispatch_matches_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/play_add_entity_clientbound_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:add_entity")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:add_entity")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundAddEntityPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "direct official ClientboundAddEntityPacket constructor with bootstrapped built-in EntityType.PIG and zero Vec3.LP movement"
        )
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_entity_id,
        oracle.answer.decoded_entity_id
    );
    assert_eq!(oracle.answer.input_uuid, oracle.answer.decoded_uuid);
    assert_eq!(
        oracle.answer.input_entity_type,
        oracle.answer.decoded_entity_type
    );
    assert_eq!(
        oracle.answer.input_entity_type.as_deref(),
        Some("minecraft:pig")
    );
    assert_eq!(oracle.answer.decoded_entity_type_registry_id, Some(100));
    assert_eq!(oracle.answer.input_x, oracle.answer.decoded_x);
    assert_eq!(oracle.answer.input_y, oracle.answer.decoded_y);
    assert_eq!(oracle.answer.input_z, oracle.answer.decoded_z);
    assert_eq!(
        oracle.answer.input_movement_x,
        oracle.answer.decoded_movement_x
    );
    assert_eq!(
        oracle.answer.input_movement_y,
        oracle.answer.decoded_movement_y
    );
    assert_eq!(
        oracle.answer.input_movement_z,
        oracle.answer.decoded_movement_z
    );
    assert_eq!(oracle.answer.encoded_movement_lp_hex.as_deref(), Some("00"));
    assert_eq!(
        oracle.answer.input_x_rot_degrees,
        oracle.answer.decoded_x_rot_degrees
    );
    assert_eq!(oracle.answer.decoded_x_rot_byte, Some(32));
    assert_eq!(
        oracle.answer.input_y_rot_degrees,
        oracle.answer.decoded_y_rot_degrees
    );
    assert_eq!(oracle.answer.decoded_y_rot_byte, Some(64));
    assert_eq!(oracle.answer.input_y_head_rot_degrees, Some(180.0));
    assert_eq!(oracle.answer.decoded_y_head_rot_degrees, Some(-180.0));
    assert_eq!(oracle.answer.decoded_y_head_rot_byte, Some(-128));
    assert_eq!(oracle.answer.input_data, oracle.answer.decoded_data);

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:add_entity",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_add_entity clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound add_entity packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Play clientbound add_entity packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound add_entity packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayAddEntityClientbound(packet) => {
            let expected_uuid: steven_protocol::protocol::UUID = oracle
                .answer
                .decoded_uuid
                .as_deref()
                .expect("play_add_entity answer missing decoded_uuid")
                .parse()
                .expect("play_add_entity answer decoded_uuid must parse as UUID");
            assert_eq!(packet.entity_id.0, oracle.answer.decoded_entity_id.unwrap());
            assert_eq!(packet.uuid, expected_uuid);
            assert_eq!(
                packet.ty.0,
                oracle.answer.decoded_entity_type_registry_id.unwrap()
            );
            assert_eq!(packet.x, oracle.answer.decoded_x.unwrap());
            assert_eq!(packet.y, oracle.answer.decoded_y.unwrap());
            assert_eq!(packet.z, oracle.answer.decoded_z.unwrap());
            assert_eq!(
                packet.movement_lp_zero.0,
                0,
                "decoded Play clientbound add_entity fixture must preserve zero Vec3.LP movement marker"
            );
            assert_eq!(packet.x_rot, oracle.answer.decoded_x_rot_byte.unwrap());
            assert_eq!(packet.y_rot, oracle.answer.decoded_y_rot_byte.unwrap());
            assert_eq!(
                packet.y_head_rot,
                oracle.answer.decoded_y_head_rot_byte.unwrap()
            );
            assert_eq!(packet.data.0, oracle.answer.decoded_data.unwrap());
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound add_entity identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound add_entity packet did not consume the official body bytes"
    );
}

