#[test]
fn play_player_info_remove_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_player_info_remove_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_player_info_remove_clientbound_framed_dispatch_body)
        .expect("spawn play_player_info_remove_clientbound oracle stack")
        .join()
        .expect("play_player_info_remove_clientbound oracle thread panicked");
}

fn play_player_info_remove_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_player_info_remove_clientbound_framed_dispatch.test-manifest.json",
        "play_player_info_remove_clientbound_framed_dispatch",
        "oracle/contracts/775/play_player_info_remove_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_player_info_remove_clientbound_framed_dispatch.answer.jsonl",
        "play_player_info_remove_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:player_info_remove",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundPlayerInfoRemovePacket")
    );
    assert_eq!(
        oracle.answer.decoded_profile_ids,
        oracle.answer.input_profile_ids
    );
    assert_eq!(
        oracle.answer.stream_decoded_profile_ids,
        oracle.answer.input_profile_ids
    );
    assert_eq!(
        oracle.answer.decoded_profile_id_count,
        oracle.answer.input_profile_id_count
    );

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound player_info_remove")
    .expect("dispatch Play clientbound player_info_remove");

    match decoded {
        packet::Packet::PlayPlayerInfoRemoveClientbound(remove) => {
            let expected: Vec<UUID> = oracle
                .answer
                .decoded_profile_ids
                .clone()
                .unwrap()
                .into_iter()
                .map(|id| id.parse().expect("parse official UUID fixture"))
                .collect();
            assert_eq!(remove.profile_ids.data, expected);
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound player_info_remove identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}

