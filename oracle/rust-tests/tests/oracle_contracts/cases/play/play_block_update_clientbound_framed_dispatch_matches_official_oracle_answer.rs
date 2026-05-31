#[test]
fn play_block_update_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_block_update_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_block_update_clientbound_framed_dispatch_body)
        .expect("spawn play_block_update_clientbound oracle stack")
        .join()
        .expect("play_block_update_clientbound oracle thread panicked");
}

fn play_block_update_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/play_block_update_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(manifest.case_id, "play_block_update_clientbound_framed_dispatch");
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/play_block_update_clientbound_framed_dispatch.contract.json"
    );
    assert_eq!(manifest.answer_path, "oracle/answers/775/play_block_update_clientbound_framed_dispatch.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "play_block_update_clientbound_framed_dispatch_matches_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/play_block_update_clientbound_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:block_update")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:block_update")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundBlockUpdatePacket BlockPos and built-in Blocks.STONE.defaultBlockState() constructor fixture; requires bootstrapped built-in block state registry but no initialized Level or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some(
            "block position encoded with BlockPos.STREAM_CODEC and block state encoded with ByteBufCodecs.idMapper(Block.BLOCK_STATE_REGISTRY)"
        )
    );
    assert_eq!(oracle.answer.input_block_x, Some(12));
    assert_eq!(oracle.answer.input_block_y, Some(64));
    assert_eq!(oracle.answer.input_block_z, Some(-7));
    assert_eq!(oracle.answer.stream_decoded_block_x, Some(12));
    assert_eq!(oracle.answer.stream_decoded_block_y, Some(64));
    assert_eq!(oracle.answer.stream_decoded_block_z, Some(-7));
    assert_eq!(oracle.answer.decoded_block_x, Some(12));
    assert_eq!(oracle.answer.decoded_block_y, Some(64));
    assert_eq!(oracle.answer.decoded_block_z, Some(-7));
    assert_eq!(
        oracle.answer.input_block.as_deref(),
        Some("minecraft:stone")
    );
    assert_eq!(
        oracle.answer.stream_decoded_block_state_block.as_deref(),
        Some("minecraft:stone")
    );
    assert_eq!(
        oracle.answer.decoded_block_state_block.as_deref(),
        Some("minecraft:stone")
    );
    assert_eq!(oracle.answer.decoded_block_state_registry_id, Some(1));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:block_update",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_block_update clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_block_update answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official block_update frame body must match the official STREAM_CODEC fixture body"
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
            "Stevenarella panicked while dispatching official Play clientbound block_update packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding Play clientbound block_update packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound block_update packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayBlockUpdateClientbound(packet) => {
            assert_eq!(packet.location.x, oracle.answer.decoded_block_x.unwrap());
            assert_eq!(packet.location.y, oracle.answer.decoded_block_y.unwrap());
            assert_eq!(packet.location.z, oracle.answer.decoded_block_z.unwrap());
            assert_eq!(
                packet.block_state.0,
                oracle.answer.decoded_block_state_registry_id.unwrap()
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound block_update identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound block_update packet did not consume the official body bytes"
    );
}

