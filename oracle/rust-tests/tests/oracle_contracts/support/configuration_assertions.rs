fn assert_finish_direction_matches_official_frame(
    official: &FinishDirectionAnswer,
    direction: Direction,
) {
    assert_eq!(official.decoded_packet_type, official.packet_type);
    let official_name_fragment = rust_name_fragment_from_packet_type(&official.packet_type);
    assert!(
        official
            .decoded_packet_class
            .contains(&official_name_fragment),
        "official decoded packet class did not preserve packet identity: {}",
        official.decoded_packet_class
    );
    assert_eq!(official.remaining_after_official_decode, 0);
    assert_eq!(
        official.instance_is_terminal, official.decoded_is_terminal,
        "official INSTANCE and decoded packet terminal flags differ for {}",
        official.flow
    );

    let expected_packet_id =
        packet_id_for(&official.configuration_packet_table, &official.packet_type);
    let framed = decode_hex(&official.encoded_framed_hex, "encoded_framed_hex");
    let body = decode_hex(&official.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Configuration,
            direction,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Configuration {} finish_configuration packet id {}",
            official.flow, framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| panic!("Stevenarella errored while decoding finish_configuration: {err}"))
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration {} finish_configuration packet id {}",
                official.flow, framed_packet_id
            )
        });
    assert!(
        format!("{decoded:?}").contains(&official_name_fragment),
        "decoded packet did not preserve finish_configuration identity: {decoded:?}"
    );
    assert!(
        body_slice.is_empty(),
        "decoded finish_configuration did not consume the official body bytes"
    );
}

fn assert_ping_pong_direction_matches_official_frame(
    official: &FramedDirectionAnswer,
    direction: Direction,
) {
    assert_eq!(official.decoded_packet_type, official.packet_type);
    assert_eq!(
        official.decoded_id, official.input_id,
        "official decoded payload id differs from input id for {}",
        official.flow
    );
    assert_eq!(official.remaining_after_official_decode, 0);

    let official_name_fragment = rust_name_fragment_from_packet_type(&official.packet_type);
    assert!(
        official
            .decoded_packet_class
            .contains(&official_name_fragment),
        "official decoded packet class did not preserve packet identity: {}",
        official.decoded_packet_class
    );

    let expected_packet_id =
        packet_id_for(&official.configuration_packet_table, &official.packet_type);
    let framed = decode_hex(&official.encoded_framed_hex, "encoded_framed_hex");
    let body = decode_hex(&official.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Configuration,
            direction,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Configuration {} {} packet id {}",
            official.flow, official.packet_type, framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| panic!("Stevenarella errored while decoding ping/pong packet: {err}"))
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration {} {} packet id {}",
                official.flow, official.packet_type, framed_packet_id
            )
        });
    let decoded_debug = format!("{decoded:?}");
    assert!(
        decoded_debug.contains(&official_name_fragment),
        "decoded packet did not preserve ping/pong identity: {decoded_debug}"
    );
    assert!(
        decoded_debug.contains(&official.input_id.to_string()),
        "decoded packet did not expose the official ping/pong payload id {}: {decoded_debug}",
        official.input_id
    );
    assert!(
        body_slice.is_empty(),
        "decoded ping/pong packet did not consume the official body bytes"
    );
}

fn rust_name_fragment_from_packet_type(packet_type: &str) -> String {
    packet_type
        .rsplit(':')
        .next()
        .unwrap_or(packet_type)
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<String>()
}
