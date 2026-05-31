#[test]
fn play_select_advancements_tab_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_select_advancements_tab_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_select_advancements_tab_clientbound_framed_dispatch_body)
        .expect("spawn play_select_advancements_tab_clientbound oracle stack")
        .join()
        .expect("play_select_advancements_tab_clientbound oracle thread panicked");
}

fn play_select_advancements_tab_clientbound_framed_dispatch_body() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_select_advancements_tab_clientbound_framed_dispatch.test-manifest.json",
        "play_select_advancements_tab_clientbound_framed_dispatch",
        "oracle/contracts/775/play_select_advancements_tab_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_select_advancements_tab_clientbound_framed_dispatch.answer.jsonl",
        "play_select_advancements_tab_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:select_advancements_tab",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundSelectAdvancementsTabPacket")
    );
    assert_eq!(oracle.answer.decoded_tab, oracle.answer.input_tab);
    assert_eq!(oracle.answer.stream_decoded_tab, oracle.answer.input_tab);

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound select_advancements_tab")
    .expect("dispatch Play clientbound select_advancements_tab");

    match decoded {
        packet::Packet::SelectAdvancementTab(tab) => {
            assert!(tab.has_id);
            assert_eq!(tab.tab_id, oracle.answer.decoded_tab.clone().unwrap());
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound select_advancements_tab identity: {other:?}")
        }
    }
    assert!(body_slice.is_empty());
}

