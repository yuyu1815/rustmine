#[test]
fn play_open_book_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_open_book_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_open_book_clientbound_framed_dispatch_body)
        .expect("spawn play_open_book_clientbound oracle stack")
        .join()
        .expect("play_open_book_clientbound oracle thread panicked");
}

fn play_open_book_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_open_book_clientbound_framed_dispatch.test-manifest.json",
        "play_open_book_clientbound_framed_dispatch",
        "oracle/contracts/775/play_open_book_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_open_book_clientbound_framed_dispatch.answer.jsonl",
        "play_open_book_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:open_book",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundOpenBookPacket")
    );
    assert_eq!(oracle.answer.input_hand, oracle.answer.decoded_hand);
    assert_eq!(
        oracle.answer.stream_decoded_hand,
        oracle.answer.decoded_hand
    );
    assert_eq!(
        oracle.answer.input_hand_ordinal,
        oracle.answer.decoded_hand_ordinal
    );
    assert_eq!(
        oracle.answer.stream_decoded_hand_ordinal,
        oracle.answer.decoded_hand_ordinal
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound open_book")
    .expect("dispatch Play clientbound open_book");

    match decoded {
        packet::Packet::OpenBook(open_book) => {
            assert_eq!(
                open_book.hand.0,
                oracle.answer.decoded_hand_ordinal.unwrap()
            );
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound open_book identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}

