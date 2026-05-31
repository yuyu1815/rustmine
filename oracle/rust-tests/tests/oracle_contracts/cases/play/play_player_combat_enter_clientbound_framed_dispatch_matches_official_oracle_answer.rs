#[test]
fn play_player_combat_enter_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_player_combat_enter_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_player_combat_enter_clientbound_framed_dispatch_body)
        .expect("spawn play_player_combat_enter_clientbound oracle stack")
        .join()
        .expect("play_player_combat_enter_clientbound oracle thread panicked");
}

fn play_player_combat_enter_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_player_combat_enter_clientbound_framed_dispatch.test-manifest.json",
        "play_player_combat_enter_clientbound_framed_dispatch",
        "oracle/contracts/775/play_player_combat_enter_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_player_combat_enter_clientbound_framed_dispatch.answer.jsonl",
        "play_player_combat_enter_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:player_combat_enter",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundPlayerCombatEnterPacket")
    );
    assert_eq!(oracle.answer.encoded_body_hex, "");
    assert_eq!(oracle.answer.decoded_same_instance, Some(true));

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound player_combat_enter")
    .expect("dispatch Play clientbound player_combat_enter");

    match decoded {
        packet::Packet::PlayPlayerCombatEnterClientbound(combat_enter) => {
            assert_eq!(combat_enter.empty, ());
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound player_combat_enter identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}

