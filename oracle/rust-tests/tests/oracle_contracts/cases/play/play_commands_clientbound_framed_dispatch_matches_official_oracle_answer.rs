#[test]
fn play_commands_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_commands_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_commands_clientbound_framed_dispatch_body)
        .expect("spawn play_commands_clientbound oracle stack")
        .join()
        .expect("play_commands_clientbound oracle thread panicked");
}

fn play_commands_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/play_commands_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(manifest.case_id, "play_commands_clientbound_framed_dispatch");
    assert_eq!(manifest.contract_path, "oracle/contracts/775/play_commands_clientbound_framed_dispatch.contract.json");
    assert_eq!(manifest.answer_path, "oracle/answers/775/play_commands_clientbound_framed_dispatch.answer.jsonl");
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, "play_commands_clientbound_framed_dispatch_matches_official_oracle_answer");
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/play_commands_clientbound_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:commands")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:commands")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundCommandsPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundCommandsPacket(RootCommandNode<S>, NodeInspector<S>) constructor fixture with an empty Brigadier CommandDispatcher root; context-free root-only command tree with no argument nodes, command context, Level, or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some("VarInt node count, then each Entry as flags byte, VarInt child index array, optional redirect index, and node-specific payload, followed by root index VarInt; this root-only fixture has one root Entry with flags 0, zero children, no redirect, no stub payload, and root index 0")
    );
    assert_eq!(oracle.answer.input_root_child_count, Some(0));
    assert_eq!(oracle.answer.stream_decoded_entry_count, Some(1));
    assert_eq!(oracle.answer.decoded_entry_count, Some(1));
    assert_eq!(oracle.answer.stream_decoded_root_index, Some(0));
    assert_eq!(oracle.answer.decoded_root_index, Some(0));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:commands",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_commands clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_commands answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    let mut expected_body = Vec::new();
    expected_body.extend_from_slice(&encode_varint(
        oracle
            .answer
            .decoded_entry_count
            .expect("missing decoded_entry_count") as i32,
    ));
    expected_body.push(0);
    expected_body.extend_from_slice(&encode_varint(0));
    expected_body.extend_from_slice(&encode_varint(
        oracle
            .answer
            .decoded_root_index
            .expect("missing decoded_root_index"),
    ));

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official commands frame body must match the official STREAM_CODEC fixture body"
    );
    assert_eq!(
        body, expected_body,
        "official commands root-only fixture body should encode one root entry and root index zero"
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
            "Stevenarella panicked while dispatching official Play clientbound commands packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Play clientbound commands packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound commands packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::DeclareCommands(commands) => {
            assert_eq!(
                commands.nodes.data.len(),
                oracle
                    .answer
                    .decoded_entry_count
                    .expect("missing decoded_entry_count")
            );
            assert_eq!(
                commands.root_index.0,
                oracle
                    .answer
                    .decoded_root_index
                    .expect("missing decoded_root_index")
            );
            let root = commands
                .nodes
                .data
                .first()
                .expect("official commands fixture should contain a root node");
            assert_eq!(root.flags, 0);
            assert!(
                root.children.data.is_empty(),
                "root-only commands fixture must not contain child nodes"
            );
            assert!(root.redirect_node.is_none());
            assert!(root.name.is_none());
            assert!(root.parser.is_none());
            assert!(root.properties.is_none());
            assert!(root.suggestions_type.is_none());
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound commands identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound commands packet did not consume the official body bytes"
    );
}

