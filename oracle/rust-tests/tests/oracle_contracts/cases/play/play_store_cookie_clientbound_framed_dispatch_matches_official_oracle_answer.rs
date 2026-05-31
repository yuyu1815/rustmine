#[test]
fn play_store_cookie_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_store_cookie_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_store_cookie_clientbound_framed_dispatch_body)
        .expect("spawn play_store_cookie_clientbound oracle stack")
        .join()
        .expect("play_store_cookie_clientbound oracle thread panicked");
}

fn play_store_cookie_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_store_cookie_clientbound_framed_dispatch.test-manifest.json",
        "play_store_cookie_clientbound_framed_dispatch",
        "oracle/contracts/775/play_store_cookie_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_store_cookie_clientbound_framed_dispatch.answer.jsonl",
        "play_store_cookie_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:store_cookie",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundStoreCookiePacket")
    );
    assert_eq!(oracle.answer.input_key, oracle.answer.stream_decoded_key);
    assert_eq!(oracle.answer.input_key, oracle.answer.decoded_key);
    assert_eq!(
        oracle.answer.input_payload_hex,
        oracle.answer.stream_decoded_payload_hex
    );
    assert_eq!(
        oracle.answer.input_payload_hex,
        oracle.answer.decoded_payload_hex
    );
    assert_eq!(
        oracle.answer.input_payload_length,
        oracle.answer.stream_decoded_payload_length
    );
    assert_eq!(
        oracle.answer.input_payload_length,
        oracle.answer.decoded_payload_length
    );
    assert_eq!(oracle.answer.stream_decoded_payload_equals_input, Some(true));
    assert_eq!(oracle.answer.decoded_payload_equals_input, Some(true));

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound store_cookie")
    .expect("dispatch Play clientbound store_cookie");

    match decoded {
        packet::Packet::PlayStoreCookieClientbound(store_cookie) => {
            assert_eq!(
                Some(store_cookie.key),
                oracle.answer.decoded_key,
                "decoded Play store_cookie key differs from official answer"
            );
            let expected_payload = decode_hex(
                oracle
                    .answer
                    .decoded_payload_hex
                    .as_deref()
                    .expect("store_cookie answer missing decoded_payload_hex"),
                "decoded_payload_hex",
            );
            assert_eq!(
                store_cookie.payload.data, expected_payload,
                "decoded Play store_cookie payload differs from official answer"
            );
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound store_cookie identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}
