#[test]
fn play_update_recipes_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let (oracle, framed_packet_id, body, _) = read_play_clientbound_oracle(
        "oracle/test-manifests/775/play_update_recipes_clientbound_framed_dispatch.test-manifest.json",
        "play_update_recipes_clientbound_framed_dispatch",
        "oracle/contracts/775/play_update_recipes_clientbound_framed_dispatch.contract.json",
        "oracle/answers/775/play_update_recipes_clientbound_framed_dispatch.answer.jsonl",
        "play_update_recipes_clientbound_framed_dispatch_matches_official_oracle_answer",
        "framed_dispatch_decode",
        "minecraft:update_recipes",
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundUpdateRecipesPacket")
    );
    assert_eq!(oracle.answer.decoded_item_set_count, Some(0));
    assert_eq!(oracle.answer.decoded_stonecutter_recipe_count, Some(0));
    assert_eq!(body, decode_hex("0000", "expected update_recipes body"));

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Play,
        Direction::Clientbound,
        framed_packet_id,
        &mut body_slice,
    )
    .expect("decode Play clientbound update_recipes")
    .expect("dispatch Play clientbound update_recipes");

    match decoded {
        packet::Packet::PlayUpdateRecipesClientbound(update_recipes) => {
            assert_eq!(update_recipes.item_set_count.0, 0);
            assert_eq!(update_recipes.stonecutter_recipe_count.0, 0);
        }
        other => panic!(
            "decoded packet did not preserve Play clientbound update_recipes identity: {other:?}"
        ),
    }
    assert!(body_slice.is_empty());
}
