#[test]
fn play_boss_event_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_boss_event_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_boss_event_clientbound_framed_dispatch_body)
        .expect("spawn play_boss_event_clientbound oracle stack")
        .join()
        .expect("play_boss_event_clientbound oracle thread panicked");
}

fn play_boss_event_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/play_boss_event_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(manifest.case_id, "play_boss_event_clientbound_framed_dispatch");
    assert_eq!(manifest.contract_path, "oracle/contracts/775/play_boss_event_clientbound_framed_dispatch.contract.json");
    assert_eq!(manifest.answer_path, "oracle/answers/775/play_boss_event_clientbound_framed_dispatch.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "play_boss_event_clientbound_framed_dispatch_matches_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/play_boss_event_clientbound_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:boss_event")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:boss_event")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundBossEventPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundBossEventPacket.createRemovePacket(UUID) fixture; context-free remove operation with no initialized BossEvent, Level, or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some(
            "UUID encoded with RegistryFriendlyByteBuf.writeUUID, operation encoded with RegistryFriendlyByteBuf.writeEnum/readEnum, and REMOVE operation writes no additional body"
        )
    );
    assert_eq!(oracle.answer.input_uuid, oracle.answer.body_uuid);
    assert_eq!(oracle.answer.input_uuid, oracle.answer.stream_decoded_uuid);
    assert_eq!(oracle.answer.input_uuid, oracle.answer.decoded_uuid);
    assert_eq!(oracle.answer.input_operation.as_deref(), Some("REMOVE"));
    assert_eq!(
        oracle.answer.stream_decoded_operation.as_deref(),
        Some("REMOVE")
    );
    assert_eq!(oracle.answer.decoded_operation.as_deref(), Some("REMOVE"));
    assert_eq!(oracle.answer.decoded_operation_ordinal, Some(1));
    assert_eq!(oracle.answer.remaining_after_operation_read, Some(0));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:boss_event",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_boss_event clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_boss_event answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official boss_event frame body must match the official STREAM_CODEC fixture body"
    );

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
            "Stevenarella panicked while dispatching official Play clientbound boss_event packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Play clientbound boss_event packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound boss_event packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::BossBar(packet) => {
            let expected_uuid: steven_protocol::protocol::UUID = oracle
                .answer
                .decoded_uuid
                .as_deref()
                .expect("play_boss_event answer missing decoded_uuid")
                .parse()
                .expect("play_boss_event answer decoded_uuid must parse as UUID");
            assert_eq!(packet.uuid, expected_uuid);
            assert_eq!(
                packet.action.0,
                oracle.answer.decoded_operation_ordinal.unwrap()
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound boss_event identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound boss_event packet did not consume the official body bytes"
    );
}

