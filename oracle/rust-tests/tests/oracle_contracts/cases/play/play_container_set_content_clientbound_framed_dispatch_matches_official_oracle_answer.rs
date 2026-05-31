#[test]
fn play_container_set_content_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_container_set_content_clientbound_oracle".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(play_container_set_content_clientbound_framed_dispatch_body)
        .expect("spawn play_container_set_content_clientbound oracle stack")
        .join()
        .expect("play_container_set_content_clientbound oracle thread panicked");
}

fn play_container_set_content_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/play_container_set_content_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(
        manifest.case_id,
        "play_container_set_content_clientbound_framed_dispatch"
    );
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/play_container_set_content_clientbound_framed_dispatch.contract.json"
    );
    assert_eq!(
        manifest.answer_path,
        "oracle/answers/775/play_container_set_content_clientbound_framed_dispatch.answer.jsonl"
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "play_container_set_content_clientbound_framed_dispatch_matches_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope("oracle/test-manifests/775/play_container_set_content_clientbound_framed_dispatch.test-manifest.json", &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:container_set_content")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:container_set_content")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundContainerSetContentPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundContainerSetContentPacket(int, int, List<ItemStack>, ItemStack) constructor fixture with an empty item list and ItemStack.EMPTY carried item; no initialized Menu, screen, Level, inventory, item registry entry, or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some("containerId encoded by ByteBufCodecs.CONTAINER_ID, stateId encoded by ByteBufCodecs.VAR_INT, items encoded by ItemStack.OPTIONAL_LIST_STREAM_CODEC as a VarInt list length followed by optional ItemStack entries, and carriedItem encoded by ItemStack.OPTIONAL_STREAM_CODEC; this fixture uses list length 0 and empty carried stack")
    );
    assert_eq!(oracle.answer.input_container_id, Some(7));
    assert_eq!(oracle.answer.stream_decoded_container_id, Some(7));
    assert_eq!(oracle.answer.decoded_container_id, Some(7));
    assert_eq!(oracle.answer.input_state_id, Some(123));
    assert_eq!(oracle.answer.stream_decoded_state_id, Some(123));
    assert_eq!(oracle.answer.decoded_state_id, Some(123));
    assert_eq!(oracle.answer.input_item_count, Some(0));
    assert_eq!(oracle.answer.stream_decoded_item_count, Some(0));
    assert_eq!(oracle.answer.decoded_item_count, Some(0));
    assert_eq!(oracle.answer.input_carried_item_empty, Some(true));
    assert_eq!(oracle.answer.stream_decoded_carried_item_empty, Some(true));
    assert_eq!(oracle.answer.decoded_carried_item_empty, Some(true));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:container_set_content",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_container_set_content clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_container_set_content answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official container_set_content frame body must match the official STREAM_CODEC fixture body"
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
            "Stevenarella panicked while dispatching official Play clientbound container_set_content packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding Play clientbound container_set_content packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound container_set_content packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayContainerSetContentClientbound(set_content) => {
            assert_eq!(
                set_content.container_id.0,
                oracle
                    .answer
                    .decoded_container_id
                    .expect("missing decoded_container_id")
            );
            assert_eq!(
                set_content.state_id.0,
                oracle
                    .answer
                    .decoded_state_id
                    .expect("missing decoded_state_id")
            );
            assert_eq!(
                set_content.items.data.len(),
                oracle
                    .answer
                    .decoded_item_count
                    .expect("missing decoded_item_count")
            );
            assert!(
                set_content.carried_item.is_none(),
                "empty official carried ItemStack should decode to None in Stevenarella"
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound container_set_content identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound container_set_content packet did not consume the official body bytes"
    );
}

