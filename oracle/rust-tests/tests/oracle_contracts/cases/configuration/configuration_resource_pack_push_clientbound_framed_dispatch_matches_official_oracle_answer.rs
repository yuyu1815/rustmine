#[test]
fn configuration_resource_pack_push_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json("oracle/test-manifests/775/configuration_resource_pack_push_clientbound_framed_dispatch.test-manifest.json");
    assert_eq!(
        manifest.case_id,
        "configuration_resource_pack_push_clientbound_framed_dispatch"
    );
    assert_eq!(
        manifest.contract_path,
        "oracle/contracts/775/configuration_resource_pack_push_clientbound_framed_dispatch.contract.json"
    );
    assert_eq!(
        manifest.answer_path,
        "oracle/answers/775/configuration_resource_pack_push_clientbound_framed_dispatch.answer.jsonl"
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        "configuration_resource_pack_push_clientbound_framed_dispatch_matches_official_oracle_answer"
    );
    assert_eq!(
        manifest.comparison_surface,
        "framed_dispatch_decode"
    );
    assert_runner_scope(
        "oracle/test-manifests/775/configuration_resource_pack_push_clientbound_framed_dispatch.test-manifest.json",
        &manifest,
    );

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:resource_pack_push")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:resource_pack_push")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundResourcePackPushPacket")
    );
    assert_eq!(
        oracle.answer.input_uuid, oracle.answer.decoded_uuid,
        "official decoded resource_pack_push UUID differs from the official input UUID"
    );
    assert_eq!(
        oracle.answer.input_url, oracle.answer.decoded_url,
        "official decoded resource_pack_push URL differs from the official input URL"
    );
    assert_eq!(
        oracle.answer.input_hash, oracle.answer.decoded_hash,
        "official decoded resource_pack_push hash differs from the official input hash"
    );
    assert_eq!(
        oracle.answer.input_required, oracle.answer.decoded_required,
        "official decoded resource_pack_push required flag differs from the official input required flag"
    );
    assert_eq!(oracle.answer.input_prompt_present, Some(false));
    assert_eq!(
        oracle.answer.input_prompt_present, oracle.answer.decoded_prompt_present,
        "official decoded resource_pack_push prompt presence differs from the official input prompt presence"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:resource_pack_push",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("resource_pack_push answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert!(
        !body.is_empty(),
        "official resource_pack_push body should include UUID, URL, hash, required flag, and prompt presence"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Configuration,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Configuration clientbound resource_pack_push packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound resource_pack_push packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound resource_pack_push packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "ResourcePackPush",
                "decoded packet did not preserve resource_pack_push compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded resource_pack_push compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!("decoded packet did not preserve clientbound resource_pack_push identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound resource_pack_push packet did not consume the official body bytes"
    );
}

