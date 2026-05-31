#[test]
fn play_command_suggestions_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_command_suggestions_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_command_suggestions_clientbound_framed_dispatch_body)
        .expect("spawn play_command_suggestions_clientbound oracle stack")
        .join()
        .expect("play_command_suggestions_clientbound oracle thread panicked");
}

fn play_command_suggestions_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/play_command_suggestions_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(
        manifest.case_id,
        "play_command_suggestions_clientbound_framed_dispatch"
    );
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/play_command_suggestions_clientbound_framed_dispatch.contract.json"
    );
    assert_eq!(
        manifest.answer_path,
        "oracle/answers/775/play_command_suggestions_clientbound_framed_dispatch.answer.jsonl"
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "play_command_suggestions_clientbound_framed_dispatch_matches_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/play_command_suggestions_clientbound_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:command_suggestions")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:command_suggestions")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundCommandSuggestionsPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundCommandSuggestionsPacket(int, Suggestions) constructor fixture with command id, StringRange.between(rangeStart, rangeEnd), and an empty suggestion list; context-free Brigadier suggestions body with no command tree, command context, Level, or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some("command id VarInt, range start VarInt, range length VarInt, then a VarInt suggestion count followed by Entry records; each Entry is text STRING_UTF8 plus optional trusted Component tooltip, and this fixture uses zero entries")
    );
    assert_eq!(oracle.answer.input_command_id, Some(123));
    assert_eq!(oracle.answer.stream_decoded_command_id, Some(123));
    assert_eq!(oracle.answer.decoded_command_id, Some(123));
    assert_eq!(oracle.answer.input_range_start, Some(1));
    assert_eq!(oracle.answer.stream_decoded_range_start, Some(1));
    assert_eq!(oracle.answer.decoded_range_start, Some(1));
    assert_eq!(oracle.answer.input_range_length, Some(3));
    assert_eq!(oracle.answer.stream_decoded_range_length, Some(3));
    assert_eq!(oracle.answer.decoded_range_length, Some(3));
    assert_eq!(oracle.answer.input_suggestion_count, Some(0));
    assert_eq!(oracle.answer.stream_decoded_suggestion_count, Some(0));
    assert_eq!(oracle.answer.decoded_suggestion_count, Some(0));
    assert_eq!(oracle.answer.decoded_to_suggestions_range_start, Some(1));
    assert_eq!(oracle.answer.decoded_to_suggestions_range_length, Some(3));
    assert_eq!(oracle.answer.decoded_to_suggestions_count, Some(0));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:command_suggestions",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_command_suggestions clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_command_suggestions answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    let mut expected_body = Vec::new();
    expected_body.extend_from_slice(&encode_varint(
        oracle
            .answer
            .input_command_id
            .expect("missing input_command_id"),
    ));
    expected_body.extend_from_slice(&encode_varint(
        oracle
            .answer
            .input_range_start
            .expect("missing input_range_start"),
    ));
    expected_body.extend_from_slice(&encode_varint(
        oracle
            .answer
            .input_range_length
            .expect("missing input_range_length"),
    ));
    expected_body.extend_from_slice(&encode_varint(
        oracle
            .answer
            .input_suggestion_count
            .expect("missing input_suggestion_count") as i32,
    ));

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official command_suggestions frame body must match the official STREAM_CODEC fixture body"
    );
    assert_eq!(
        body, expected_body,
        "official command_suggestions empty fixture body should encode id/start/length/count as VarInts"
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
            "Stevenarella panicked while dispatching official Play clientbound command_suggestions packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding Play clientbound command_suggestions packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound command_suggestions packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayCommandSuggestionsClientbound(command_suggestions) => {
            assert_eq!(
                command_suggestions.id.0,
                oracle
                    .answer
                    .decoded_command_id
                    .expect("missing decoded_command_id")
            );
            assert_eq!(
                command_suggestions.start.0,
                oracle
                    .answer
                    .decoded_range_start
                    .expect("missing decoded_range_start")
            );
            assert_eq!(
                command_suggestions.length.0,
                oracle
                    .answer
                    .decoded_range_length
                    .expect("missing decoded_range_length")
            );
            assert_eq!(
                command_suggestions.suggestion_count.0,
                oracle
                    .answer
                    .decoded_suggestion_count
                    .expect("missing decoded_suggestion_count") as i32
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound command_suggestions identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound command_suggestions packet did not consume the official body bytes"
    );
}

