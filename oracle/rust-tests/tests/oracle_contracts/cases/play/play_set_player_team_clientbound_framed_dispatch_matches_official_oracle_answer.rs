#[test]
fn play_set_player_team_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_set_player_team_clientbound_framed_dispatch.test-manifest.json",
        "play_set_player_team_clientbound_framed_dispatch",
        "oracle/contracts/775/play_set_player_team_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_set_player_team_clientbound_framed_dispatch.answer.jsonl",
        "play_set_player_team_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:set_player_team",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSetPlayerTeamPacket")
    );
    assert_eq!(oracle.answer.input_team_name.as_deref(), Some("rustmine_team"));
    assert_eq!(oracle.answer.input_team_name, oracle.answer.stream_decoded_team_name);
    assert_eq!(oracle.answer.input_team_name, oracle.answer.decoded_team_name);
    assert_eq!(oracle.answer.input_method, Some(1));
    assert_eq!(oracle.answer.input_team_parameters_present, Some(false));
    assert_eq!(oracle.answer.input_player_count, Some(0));

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound set_player_team")
    .expect("dispatch Play clientbound set_player_team");

    match decoded {
        packet::Packet::PlaySetPlayerTeamClientbound(team) => {
            assert_eq!(team.team_name, "rustmine_team");
            assert_eq!(team.method, 1);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound set_player_team identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());

    let mut unsupported_body = body.clone();
    *unsupported_body.last_mut().expect("method byte") = 0;
    let err = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut unsupported_body.as_slice(),
    )
    .expect_err("add/change set_player_team branches must stay unsupported");
    assert!(
        err.to_string()
            .contains("unsupported Play set_player_team method 0"),
        "unexpected error: {err}"
    );
}
