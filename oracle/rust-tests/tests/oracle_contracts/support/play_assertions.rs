fn assert_move_entity_common(oracle: &OracleAnswer, expected_class: &str) {
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some(expected_class)
    );
    assert_eq!(
        oracle.answer.input_entity_id,
        oracle.answer.decoded_entity_id
    );
    assert_eq!(
        oracle.answer.stream_decoded_entity_id,
        oracle.answer.decoded_entity_id
    );
    assert_eq!(oracle.answer.input_xa, oracle.answer.decoded_xa);
    assert_eq!(oracle.answer.stream_decoded_xa, oracle.answer.decoded_xa);
    assert_eq!(oracle.answer.input_ya, oracle.answer.decoded_ya);
    assert_eq!(oracle.answer.stream_decoded_ya, oracle.answer.decoded_ya);
    assert_eq!(oracle.answer.input_za, oracle.answer.decoded_za);
    assert_eq!(oracle.answer.stream_decoded_za, oracle.answer.decoded_za);
    assert_eq!(
        oracle.answer.input_move_y_rot_byte,
        oracle.answer.decoded_move_y_rot_byte
    );
    assert_eq!(
        oracle.answer.stream_decoded_move_y_rot_byte,
        oracle.answer.decoded_move_y_rot_byte
    );
    assert_eq!(
        oracle.answer.input_move_x_rot_byte,
        oracle.answer.decoded_move_x_rot_byte
    );
    assert_eq!(
        oracle.answer.stream_decoded_move_x_rot_byte,
        oracle.answer.decoded_move_x_rot_byte
    );
    assert_eq!(
        oracle.answer.input_on_ground,
        oracle.answer.decoded_on_ground
    );
    assert_eq!(
        oracle.answer.stream_decoded_on_ground,
        oracle.answer.decoded_on_ground
    );
    assert_eq!(
        oracle.answer.input_has_rotation,
        oracle.answer.decoded_has_rotation
    );
    assert_eq!(
        oracle.answer.stream_decoded_has_rotation,
        oracle.answer.decoded_has_rotation
    );
    assert_eq!(
        oracle.answer.input_has_position,
        oracle.answer.decoded_has_position
    );
    assert_eq!(
        oracle.answer.stream_decoded_has_position,
        oracle.answer.decoded_has_position
    );
}
