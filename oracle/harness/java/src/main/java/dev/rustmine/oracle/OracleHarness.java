package dev.rustmine.oracle;

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import com.mojang.authlib.GameProfile;
import io.netty.buffer.Unpooled;
import it.unimi.dsi.fastutil.objects.Object2IntMap;
import it.unimi.dsi.fastutil.objects.Object2IntOpenHashMap;
import java.io.IOException;
import java.lang.reflect.Constructor;
import java.lang.reflect.Field;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Instant;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HexFormat;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.Set;
import java.util.UUID;
import net.minecraft.SharedConstants;
import net.minecraft.core.BlockPos;
import net.minecraft.core.RegistryAccess;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.handshake.ClientIntent;
import net.minecraft.network.protocol.handshake.ClientIntentionPacket;
import net.minecraft.network.protocol.handshake.HandshakeProtocols;
import net.minecraft.network.protocol.handshake.ServerHandshakePacketListener;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundAddEntityPacket;
import net.minecraft.network.protocol.game.ClientboundAnimatePacket;
import net.minecraft.network.protocol.game.ClientboundAwardStatsPacket;
import net.minecraft.network.protocol.game.ClientboundBlockChangedAckPacket;
import net.minecraft.network.protocol.game.ClientboundBlockDestructionPacket;
import net.minecraft.network.protocol.game.ClientboundBlockEntityDataPacket;
import net.minecraft.network.protocol.game.ClientboundBlockEventPacket;
import net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket;
import net.minecraft.network.protocol.game.ClientboundBossEventPacket;
import net.minecraft.network.protocol.game.ClientboundChangeDifficultyPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.network.protocol.login.LoginProtocols;
import net.minecraft.network.protocol.login.ClientLoginPacketListener;
import net.minecraft.network.protocol.login.ClientboundCustomQueryPacket;
import net.minecraft.network.protocol.login.ClientboundHelloPacket;
import net.minecraft.network.protocol.login.ClientboundLoginCompressionPacket;
import net.minecraft.network.protocol.login.ClientboundLoginDisconnectPacket;
import net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket;
import net.minecraft.network.protocol.login.custom.DiscardedQueryPayload;
import net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket;
import net.minecraft.network.protocol.login.ServerLoginPacketListener;
import net.minecraft.network.protocol.login.ServerboundHelloPacket;
import net.minecraft.network.protocol.login.ServerboundKeyPacket;
import net.minecraft.network.protocol.login.ServerboundLoginAcknowledgedPacket;
import net.minecraft.network.protocol.configuration.ClientConfigurationPacketListener;
import net.minecraft.network.protocol.configuration.ClientboundCodeOfConductPacket;
import net.minecraft.network.protocol.configuration.ClientboundFinishConfigurationPacket;
import net.minecraft.network.protocol.configuration.ClientboundRegistryDataPacket;
import net.minecraft.network.protocol.configuration.ConfigurationProtocols;
import net.minecraft.network.protocol.configuration.ClientboundResetChatPacket;
import net.minecraft.network.protocol.configuration.ClientboundSelectKnownPacks;
import net.minecraft.network.protocol.configuration.ClientboundUpdateEnabledFeaturesPacket;
import net.minecraft.network.protocol.configuration.ServerboundAcceptCodeOfConductPacket;
import net.minecraft.network.protocol.configuration.ServerConfigurationPacketListener;
import net.minecraft.network.protocol.configuration.ServerboundFinishConfigurationPacket;
import net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.common.ClientboundClearDialogPacket;
import net.minecraft.network.protocol.common.ClientboundKeepAlivePacket;
import net.minecraft.network.protocol.common.ClientboundCustomPayloadPacket;
import net.minecraft.network.protocol.common.ClientboundCustomReportDetailsPacket;
import net.minecraft.network.protocol.common.ClientboundDisconnectPacket;
import net.minecraft.network.protocol.common.ClientboundPingPacket;
import net.minecraft.network.protocol.common.ClientboundResourcePackPopPacket;
import net.minecraft.network.protocol.common.ClientboundResourcePackPushPacket;
import net.minecraft.network.protocol.common.ClientboundServerLinksPacket;
import net.minecraft.network.protocol.common.ClientboundShowDialogPacket;
import net.minecraft.network.protocol.common.ClientboundStoreCookiePacket;
import net.minecraft.network.protocol.common.ClientboundTransferPacket;
import net.minecraft.network.protocol.common.ClientboundUpdateTagsPacket;
import net.minecraft.network.protocol.common.ServerboundClientInformationPacket;
import net.minecraft.network.protocol.common.ServerboundCustomClickActionPacket;
import net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket;
import net.minecraft.network.protocol.common.ServerboundKeepAlivePacket;
import net.minecraft.network.protocol.common.ServerboundPongPacket;
import net.minecraft.network.protocol.common.ServerboundResourcePackPacket;
import net.minecraft.network.protocol.common.custom.BrandPayload;
import net.minecraft.network.protocol.cookie.ClientboundCookieRequestPacket;
import net.minecraft.network.protocol.cookie.ServerboundCookieResponsePacket;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.nbt.Tag;
import net.minecraft.core.Holder;
import net.minecraft.core.Registry;
import net.minecraft.core.registries.Registries;
import net.minecraft.resources.Identifier;
import net.minecraft.resources.ResourceKey;
import net.minecraft.server.ServerLinks;
import net.minecraft.server.Bootstrap;
import net.minecraft.server.dialog.CommonDialogData;
import net.minecraft.server.dialog.Dialog;
import net.minecraft.server.dialog.DialogAction;
import net.minecraft.server.dialog.NoticeDialog;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.packs.repository.KnownPack;
import net.minecraft.stats.Stat;
import net.minecraft.tags.TagNetworkSerialization;
import net.minecraft.world.entity.EntityType;
import net.minecraft.world.Difficulty;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.entity.BlockEntityType;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.phys.Vec3;

public final class OracleHarness {
    private static final Gson GSON = new Gson();

    private OracleHarness() {
    }

    public static void main(String[] args) throws Exception {
        if (args.length != 1) {
            throw new IllegalArgumentException("expected one oracle case path");
        }

        JsonObject input = readJson(Path.of(args[0]));
        String caseId = input.get("case_id").getAsString();
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();

        if ("configuration_keepalive_codec".equals(caseId)) {
            writeAnswer(input, configurationKeepAliveCodec(input));
            return;
        }
        if ("configuration_keepalive_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationKeepAliveFramedDispatch(input));
            return;
        }
        if ("configuration_keepalive_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationKeepAliveClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_cookie_request_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationCookieRequestFramedDispatch(input));
            return;
        }
        if ("configuration_finish_framed_terminal".equals(caseId)) {
            writeAnswer(input, configurationFinishFramedTerminal(input));
            return;
        }
        if ("configuration_ping_pong_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationPingPongFramedDispatch(input));
            return;
        }
        if ("configuration_client_information_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationClientInformationFramedDispatch(input));
            return;
        }
        if ("configuration_cookie_response_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationCookieResponseFramedDispatch(input));
            return;
        }
        if ("configuration_custom_payload_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationCustomPayloadFramedDispatch(input));
            return;
        }
        if ("configuration_custom_payload_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationCustomPayloadClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_disconnect_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationDisconnectClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_reset_chat_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationResetChatClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_registry_data_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationRegistryDataClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_resource_pack_pop_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationResourcePackPopClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_resource_pack_push_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationResourcePackPushClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_store_cookie_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationStoreCookieClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_transfer_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationTransferClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_update_enabled_features_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationUpdateEnabledFeaturesClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_update_tags_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationUpdateTagsClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_select_known_packs_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationSelectKnownPacksClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_custom_report_details_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationCustomReportDetailsClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_server_links_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationServerLinksClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_clear_dialog_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationClearDialogClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_show_dialog_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationShowDialogClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_code_of_conduct_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationCodeOfConductClientboundFramedDispatch(input));
            return;
        }
        if ("configuration_resource_pack_response_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationResourcePackResponseFramedDispatch(input));
            return;
        }
        if ("configuration_select_known_packs_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationSelectKnownPacksFramedDispatch(input));
            return;
        }
        if ("configuration_custom_click_action_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationCustomClickActionFramedDispatch(input));
            return;
        }
        if ("configuration_accept_code_of_conduct_framed_dispatch".equals(caseId)) {
            writeAnswer(input, configurationAcceptCodeOfConductFramedDispatch(input));
            return;
        }
        if ("handshake_intention_framed_dispatch".equals(caseId)) {
            writeAnswer(input, handshakeIntentionFramedDispatch(input));
            return;
        }
        if ("login_hello_serverbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, loginHelloServerboundFramedDispatch(input));
            return;
        }
        if ("login_key_serverbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, loginKeyServerboundFramedDispatch(input));
            return;
        }
        if ("login_custom_query_answer_serverbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, loginCustomQueryAnswerServerboundFramedDispatch(input));
            return;
        }
        if ("login_acknowledged_serverbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, loginAcknowledgedServerboundFramedDispatch(input));
            return;
        }
        if ("login_cookie_response_serverbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, loginCookieResponseServerboundFramedDispatch(input));
            return;
        }
        if ("login_disconnect_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, loginDisconnectClientboundFramedDispatch(input));
            return;
        }
        if ("login_hello_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, loginHelloClientboundFramedDispatch(input));
            return;
        }
        if ("login_finished_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, loginFinishedClientboundFramedDispatch(input));
            return;
        }
        if ("login_compression_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, loginCompressionClientboundFramedDispatch(input));
            return;
        }
        if ("login_custom_query_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, loginCustomQueryClientboundFramedDispatch(input));
            return;
        }
        if ("login_cookie_request_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, loginCookieRequestClientboundFramedDispatch(input));
            return;
        }
        if ("play_bundle_delimiter_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, playBundleDelimiterClientboundFramedDispatch(input));
            return;
        }
        if ("play_add_entity_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, playAddEntityClientboundFramedDispatch(input));
            return;
        }
        if ("play_animate_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, playAnimateClientboundFramedDispatch(input));
            return;
        }
        if ("play_award_stats_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, playAwardStatsClientboundFramedDispatch(input));
            return;
        }
        if ("play_block_changed_ack_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, playBlockChangedAckClientboundFramedDispatch(input));
            return;
        }
        if ("play_block_destruction_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, playBlockDestructionClientboundFramedDispatch(input));
            return;
        }
        if ("play_block_entity_data_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, playBlockEntityDataClientboundFramedDispatch(input));
            return;
        }
        if ("play_block_event_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, playBlockEventClientboundFramedDispatch(input));
            return;
        }
        if ("play_block_update_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, playBlockUpdateClientboundFramedDispatch(input));
            return;
        }
        if ("play_boss_event_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, playBossEventClientboundFramedDispatch(input));
            return;
        }
        if ("play_change_difficulty_clientbound_framed_dispatch".equals(caseId)) {
            writeAnswer(input, playChangeDifficultyClientboundFramedDispatch(input));
            return;
        }

        throw new IllegalArgumentException("unsupported oracle case: " + caseId);
    }

    private static JsonObject readJson(Path path) throws IOException {
        return GSON.fromJson(Files.readString(path), JsonObject.class);
    }

    private static void writeAnswer(JsonObject input, Map<String, Object> answer) throws IOException {
        Path output = Path.of(input.get("answer_path").getAsString());
        Files.createDirectories(output.getParent());
        Files.writeString(output, GSON.toJson(answer) + System.lineSeparator());
        System.err.println("wrote " + output);
    }

    private static Map<String, Object> configurationKeepAliveCodec(JsonObject input) {
        long id = input
            .getAsJsonObject("question")
            .getAsJsonObject("input_fields")
            .get("id")
            .getAsLong();

        FriendlyByteBuf out = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundKeepAlivePacket.STREAM_CODEC.encode(out, new ServerboundKeepAlivePacket(id));
        byte[] body = new byte[out.readableBytes()];
        out.getBytes(out.readerIndex(), body);

        ServerboundKeepAlivePacket decoded = ServerboundKeepAlivePacket.STREAM_CODEC.decode(
            new FriendlyByteBuf(Unpooled.wrappedBuffer(body))
        );

        List<Map<String, Object>> configurationServerboundPackets = new ArrayList<>();
        ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationServerboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ServerboundKeepAlivePacket.STREAM_CODEC",
            "decompiled_source_path", "_analysis/minecraft-26.1.2/decompiled-protocol/net/minecraft/network/protocol/common/ServerboundKeepAlivePacket.java"
        ));
        answer.put("answer", Map.of(
            "state", "Configuration",
            "flow", "Serverbound",
            "packet_type", "minecraft:keep_alive",
            "input_id", id,
            "encoded_body_hex", HexFormat.of().formatHex(body),
            "decoded_id", decoded.getId(),
            "configuration_serverbound_packet_table", configurationServerboundPackets
        ));
        return answer;
    }

    private static Map<String, Object> handshakeIntentionFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int protocolVersion = inputFields.get("protocol_version").getAsInt();
        String host = inputFields.get("host").getAsString();
        int port = inputFields.get("port").getAsInt();
        ClientIntent intent = ClientIntent.valueOf(inputFields.get("intent").getAsString());

        ClientIntentionPacket packet = new ClientIntentionPacket(protocolVersion, host, port, intent);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        HandshakeProtocols.SERVERBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ServerHandshakePacketListener> decodedPacket =
            HandshakeProtocols.SERVERBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientIntentionPacket decodedIntention)) {
            throw new IllegalStateException(
                "expected ClientIntentionPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientIntentionPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> handshakingServerboundPackets = new ArrayList<>();
        HandshakeProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            handshakingServerboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientIntentionPacket(int, String, int, ClientIntent), ClientIntentionPacket.STREAM_CODEC, HandshakeProtocols.SERVERBOUND.codec().encode/decode(ClientIntentionPacket), HandshakeProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), ClientIntentionPacket.protocolVersion(), hostName(), port(), intention(), isTerminal()",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.handshake.ClientIntentionPacket net.minecraft.network.protocol.handshake.HandshakeProtocols net.minecraft.network.protocol.handshake.HandshakePacketTypes net.minecraft.network.protocol.handshake.ClientIntent"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Handshaking");
        answerBody.put("flow", "Serverbound");
        answerBody.put("packet_type", "minecraft:intention");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_protocol_version", protocolVersion);
        answerBody.put("decoded_protocol_version", decodedIntention.protocolVersion());
        answerBody.put("input_host", host);
        answerBody.put("decoded_host", decodedIntention.hostName());
        answerBody.put("input_port", port);
        answerBody.put("decoded_port", decodedIntention.port());
        answerBody.put("input_intent", intent.name());
        answerBody.put("decoded_intent", decodedIntention.intention().name());
        answerBody.put("input_intent_id", intent.id());
        answerBody.put("decoded_intent_id", decodedIntention.intention().id());
        answerBody.put("input_is_terminal", packet.isTerminal());
        answerBody.put("decoded_is_terminal", decodedIntention.isTerminal());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("handshaking_serverbound_packet_table", handshakingServerboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> loginHelloServerboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String name = inputFields.get("name").getAsString();
        UUID profileId = UUID.fromString(inputFields.get("profile_id").getAsString());

        ServerboundHelloPacket packet = new ServerboundHelloPacket(name, profileId);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        LoginProtocols.SERVERBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ServerLoginPacketListener> decodedPacket =
            LoginProtocols.SERVERBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ServerboundHelloPacket decodedHello)) {
            throw new IllegalStateException(
                "expected ServerboundHelloPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundHelloPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> loginServerboundPackets = new ArrayList<>();
        LoginProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            loginServerboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ServerboundHelloPacket(String, UUID), ServerboundHelloPacket.STREAM_CODEC, LoginProtocols.SERVERBOUND.codec().encode/decode(ServerboundHelloPacket), LoginProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), ServerboundHelloPacket.name(), profileId()",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ServerboundHelloPacket"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Login");
        answerBody.put("flow", "Serverbound");
        answerBody.put("packet_type", "minecraft:hello");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_name", name);
        answerBody.put("decoded_name", decodedHello.name());
        answerBody.put("input_profile_id", profileId.toString());
        answerBody.put("decoded_profile_id", decodedHello.profileId().toString());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("login_serverbound_packet_table", loginServerboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> loginKeyServerboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        byte[] keybytes = hexToBytes(inputFields.get("keybytes_hex").getAsString());
        byte[] encryptedChallenge = hexToBytes(inputFields.get("encrypted_challenge_hex").getAsString());

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        fixtureBodyOut.writeByteArray(keybytes);
        fixtureBodyOut.writeByteArray(encryptedChallenge);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        ServerboundKeyPacket packet = ServerboundKeyPacket.STREAM_CODEC.decode(
            new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody))
        );

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        LoginProtocols.SERVERBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ServerLoginPacketListener> decodedPacket =
            LoginProtocols.SERVERBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ServerboundKeyPacket decodedKey)) {
            throw new IllegalStateException(
                "expected ServerboundKeyPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundKeyPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        byte[] decodedKeybytes = privateByteArray(decodedKey, "keybytes");
        byte[] decodedEncryptedChallenge = privateByteArray(decodedKey, "encryptedChallenge");

        List<Map<String, Object>> loginServerboundPackets = new ArrayList<>();
        LoginProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            loginServerboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ServerboundKeyPacket.STREAM_CODEC, LoginProtocols.SERVERBOUND.codec().encode/decode(ServerboundKeyPacket), LoginProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), private ServerboundKeyPacket(FriendlyByteBuf), private write(FriendlyByteBuf), keybytes, encryptedChallenge",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ServerboundKeyPacket"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Login");
        answerBody.put("flow", "Serverbound");
        answerBody.put("packet_type", "minecraft:key");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_keybytes_hex", HexFormat.of().formatHex(keybytes));
        answerBody.put("decoded_keybytes_hex", HexFormat.of().formatHex(decodedKeybytes));
        answerBody.put("input_keybytes_length", keybytes.length);
        answerBody.put("decoded_keybytes_length", decodedKeybytes.length);
        answerBody.put("input_encrypted_challenge_hex", HexFormat.of().formatHex(encryptedChallenge));
        answerBody.put("decoded_encrypted_challenge_hex", HexFormat.of().formatHex(decodedEncryptedChallenge));
        answerBody.put("input_encrypted_challenge_length", encryptedChallenge.length);
        answerBody.put("decoded_encrypted_challenge_length", decodedEncryptedChallenge.length);
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("login_serverbound_packet_table", loginServerboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> loginCustomQueryAnswerServerboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int transactionId = inputFields.get("transaction_id").getAsInt();
        boolean payloadPresent = inputFields.get("payload_present").getAsBoolean();
        if (payloadPresent) {
            throw new IllegalArgumentException("this oracle fixture is scoped to the null custom query answer payload");
        }

        ServerboundCustomQueryAnswerPacket packet =
            new ServerboundCustomQueryAnswerPacket(transactionId, null);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        LoginProtocols.SERVERBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ServerLoginPacketListener> decodedPacket =
            LoginProtocols.SERVERBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ServerboundCustomQueryAnswerPacket decodedCustomQueryAnswer)) {
            throw new IllegalStateException(
                "expected ServerboundCustomQueryAnswerPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundCustomQueryAnswerPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> loginServerboundPackets = new ArrayList<>();
        LoginProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            loginServerboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ServerboundCustomQueryAnswerPacket(int, CustomQueryAnswerPayload), ServerboundCustomQueryAnswerPacket.STREAM_CODEC, LoginProtocols.SERVERBOUND.codec().encode/decode(ServerboundCustomQueryAnswerPacket), LoginProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), transactionId(), payload(), readPayload(...), write(FriendlyByteBuf)",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Login");
        answerBody.put("flow", "Serverbound");
        answerBody.put("packet_type", "minecraft:custom_query_answer");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_transaction_id", transactionId);
        answerBody.put("decoded_transaction_id", decodedCustomQueryAnswer.transactionId());
        answerBody.put("input_payload_present", payloadPresent);
        answerBody.put("decoded_payload_present", decodedCustomQueryAnswer.payload() != null);
        answerBody.put(
            "decoded_payload_class",
            decodedCustomQueryAnswer.payload() == null ? null : decodedCustomQueryAnswer.payload().getClass().getName()
        );
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("login_serverbound_packet_table", loginServerboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> loginDisconnectClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String reasonText = inputFields.get("reason_text").getAsString();
        Component reason = Component.literal(reasonText);
        ClientboundLoginDisconnectPacket packet = new ClientboundLoginDisconnectPacket(reason);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        LoginProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientLoginPacketListener> decodedPacket =
            LoginProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundLoginDisconnectPacket decodedDisconnect)) {
            throw new IllegalStateException(
                "expected ClientboundLoginDisconnectPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundLoginDisconnectPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> loginClientboundPackets = new ArrayList<>();
        LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            loginClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "Component.literal(String), ClientboundLoginDisconnectPacket(Component), ClientboundLoginDisconnectPacket.STREAM_CODEC, LoginProtocols.CLIENTBOUND.codec().encode/decode(ClientboundLoginDisconnectPacket), LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundLoginDisconnectPacket.reason(), Component.getString(), ClientLoginPacketListener extends ClientCookiePacketListener",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ClientboundLoginDisconnectPacket net.minecraft.network.protocol.login.ClientLoginPacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Login");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:login_disconnect");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("reason_fixture", "Component.literal(\"\")");
        answerBody.put("input_reason_text", reason.getString());
        answerBody.put("decoded_reason_text", decodedDisconnect.reason().getString());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("login_clientbound_packet_table", loginClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationKeepAliveFramedDispatch(JsonObject input) {
        long id = input
            .getAsJsonObject("question")
            .getAsJsonObject("input_fields")
            .get("id")
            .getAsLong();

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.SERVERBOUND
            .codec()
            .encode(framedOut, new ServerboundKeepAlivePacket(id));
        byte[] framed = new byte[framedOut.readableBytes()];
        framedOut.getBytes(framedOut.readerIndex(), framed);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ServerConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.SERVERBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ServerboundKeepAlivePacket decodedKeepAlive)) {
            throw new IllegalStateException(
                "expected ServerboundKeepAlivePacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundKeepAlivePacket.STREAM_CODEC.encode(bodyOut, new ServerboundKeepAlivePacket(id));
        byte[] body = new byte[bodyOut.readableBytes()];
        bodyOut.getBytes(bodyOut.readerIndex(), body);

        List<Map<String, Object>> configurationServerboundPackets = new ArrayList<>();
        ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationServerboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ConfigurationProtocols.SERVERBOUND.codec().encode(...), ConfigurationProtocols.SERVERBOUND.codec().decode(...), ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), ServerboundKeepAlivePacket.getId()",
            "decompiled_source_path", "_analysis/minecraft-26.1.2/decompiled-protocol/net/minecraft/network/protocol/configuration/ConfigurationProtocols.java"
        ));
        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Serverbound");
        answerBody.put("packet_type", "minecraft:keep_alive");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_id", id);
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("decoded_id", decodedKeepAlive.getId());
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_serverbound_packet_table", configurationServerboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationKeepAliveClientboundFramedDispatch(JsonObject input) {
        long id = input
            .getAsJsonObject("question")
            .getAsJsonObject("input_fields")
            .get("id")
            .getAsLong();

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND
            .codec()
            .encode(framedOut, new ClientboundKeepAlivePacket(id));
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundKeepAlivePacket decodedKeepAlive)) {
            throw new IllegalStateException(
                "expected ClientboundKeepAlivePacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundKeepAlivePacket.STREAM_CODEC.encode(bodyOut, new ClientboundKeepAlivePacket(id));
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ConfigurationProtocols.CLIENTBOUND.codec().encode(...), ConfigurationProtocols.CLIENTBOUND.codec().decode(...), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundKeepAlivePacket.getId()",
            "decompiled_source_path", "_analysis/minecraft-26.1.2/decompiled-protocol/net/minecraft/network/protocol/configuration/ConfigurationProtocols.java"
        ));
        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:keep_alive");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_id", id);
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("decoded_id", decodedKeepAlive.getId());
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationCookieRequestFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        Identifier key = Identifier.parse(inputFields.get("key").getAsString());
        ClientboundCookieRequestPacket packet = new ClientboundCookieRequestPacket(key);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundCookieRequestPacket decodedCookieRequest)) {
            throw new IllegalStateException(
                "expected ClientboundCookieRequestPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundCookieRequestPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "Identifier.parse(String), ClientboundCookieRequestPacket(Identifier), ClientboundCookieRequestPacket.STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundCookieRequestPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundCookieRequestPacket.key()",
            "decompiled_source_path", "_analysis/minecraft-26.1.2/decompiled-protocol/net/minecraft/network/protocol/cookie/ClientboundCookieRequestPacket.java"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:cookie_request");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_key", key.toString());
        answerBody.put("decoded_key", decodedCookieRequest.key().toString());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationFinishFramedTerminal(JsonObject input) {
        FriendlyByteBuf serverboundOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.SERVERBOUND
            .codec()
            .encode(serverboundOut, ServerboundFinishConfigurationPacket.INSTANCE);
        byte[] serverboundFramed = readableBytes(serverboundOut);

        FriendlyByteBuf serverboundIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(serverboundFramed));
        Packet<? super ServerConfigurationPacketListener> serverboundDecoded =
            ConfigurationProtocols.SERVERBOUND.codec().decode(serverboundIn);
        if (!(serverboundDecoded instanceof ServerboundFinishConfigurationPacket)) {
            throw new IllegalStateException(
                "expected ServerboundFinishConfigurationPacket, got " + serverboundDecoded.getClass().getName()
            );
        }

        FriendlyByteBuf clientboundOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND
            .codec()
            .encode(clientboundOut, ClientboundFinishConfigurationPacket.INSTANCE);
        byte[] clientboundFramed = readableBytes(clientboundOut);

        FriendlyByteBuf clientboundIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(clientboundFramed));
        Packet<? super ClientConfigurationPacketListener> clientboundDecoded =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(clientboundIn);
        if (!(clientboundDecoded instanceof ClientboundFinishConfigurationPacket)) {
            throw new IllegalStateException(
                "expected ClientboundFinishConfigurationPacket, got " + clientboundDecoded.getClass().getName()
            );
        }

        List<Map<String, Object>> configurationServerboundPackets = new ArrayList<>();
        ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationServerboundPackets.add(row);
        });

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundFinishConfigurationPacket.INSTANCE), ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundFinishConfigurationPacket.INSTANCE), ServerboundFinishConfigurationPacket.INSTANCE.isTerminal(), ClientboundFinishConfigurationPacket.INSTANCE.isTerminal()",
            "decompiled_source_path", "_analysis/minecraft-26.1.2/decompiled-protocol/net/minecraft/network/protocol/configuration/ConfigurationProtocols.java"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("packet_type", "minecraft:finish_configuration");
        answerBody.put(
            "serverbound",
            finishDirectionAnswer(
                "Serverbound",
                "minecraft:finish_configuration",
                serverboundDecoded,
                ServerboundFinishConfigurationPacket.INSTANCE.isTerminal(),
                serverboundDecoded.isTerminal(),
                serverboundFramed,
                serverboundIn.readableBytes(),
                configurationServerboundPackets
            )
        );
        answerBody.put(
            "clientbound",
            finishDirectionAnswer(
                "Clientbound",
                "minecraft:finish_configuration",
                clientboundDecoded,
                ClientboundFinishConfigurationPacket.INSTANCE.isTerminal(),
                clientboundDecoded.isTerminal(),
                clientboundFramed,
                clientboundIn.readableBytes(),
                configurationClientboundPackets
            )
        );
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationPingPongFramedDispatch(JsonObject input) {
        int id = input
            .getAsJsonObject("question")
            .getAsJsonObject("input_fields")
            .get("id")
            .getAsInt();

        FriendlyByteBuf clientboundOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND
            .codec()
            .encode(clientboundOut, new ClientboundPingPacket(id));
        byte[] clientboundFramed = readableBytes(clientboundOut);

        FriendlyByteBuf clientboundIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(clientboundFramed));
        Packet<? super ClientConfigurationPacketListener> clientboundDecoded =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(clientboundIn);
        if (!(clientboundDecoded instanceof ClientboundPingPacket decodedPing)) {
            throw new IllegalStateException(
                "expected ClientboundPingPacket, got " + clientboundDecoded.getClass().getName()
            );
        }

        FriendlyByteBuf clientboundBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundPingPacket.STREAM_CODEC.encode(clientboundBodyOut, new ClientboundPingPacket(id));
        byte[] clientboundBody = readableBytes(clientboundBodyOut);

        FriendlyByteBuf serverboundOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.SERVERBOUND
            .codec()
            .encode(serverboundOut, new ServerboundPongPacket(id));
        byte[] serverboundFramed = readableBytes(serverboundOut);

        FriendlyByteBuf serverboundIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(serverboundFramed));
        Packet<? super ServerConfigurationPacketListener> serverboundDecoded =
            ConfigurationProtocols.SERVERBOUND.codec().decode(serverboundIn);
        if (!(serverboundDecoded instanceof ServerboundPongPacket decodedPong)) {
            throw new IllegalStateException(
                "expected ServerboundPongPacket, got " + serverboundDecoded.getClass().getName()
            );
        }

        FriendlyByteBuf serverboundBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundPongPacket.STREAM_CODEC.encode(serverboundBodyOut, new ServerboundPongPacket(id));
        byte[] serverboundBody = readableBytes(serverboundBodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        List<Map<String, Object>> configurationServerboundPackets = new ArrayList<>();
        ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationServerboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundPingPacket), ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundPongPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), ClientboundPingPacket.getId(), ServerboundPongPacket.getId()",
            "decompiled_source_path", "_analysis/minecraft-26.1.2/decompiled-protocol/net/minecraft/network/protocol/configuration/ConfigurationProtocols.java"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put(
            "clientbound_ping",
            framedDirectionAnswer(
                "Clientbound",
                "minecraft:ping",
                clientboundDecoded,
                id,
                decodedPing.getId(),
                clientboundFramed,
                clientboundBody,
                clientboundIn.readableBytes(),
                configurationClientboundPackets
            )
        );
        answerBody.put(
            "serverbound_pong",
            framedDirectionAnswer(
                "Serverbound",
                "minecraft:pong",
                serverboundDecoded,
                id,
                decodedPong.getId(),
                serverboundFramed,
                serverboundBody,
                serverboundIn.readableBytes(),
                configurationServerboundPackets
            )
        );
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationClientInformationFramedDispatch(JsonObject input) {
        ClientInformation information = ClientInformation.createDefault();
        ServerboundClientInformationPacket packet = new ServerboundClientInformationPacket(information);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.SERVERBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ServerConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.SERVERBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ServerboundClientInformationPacket decodedClientInformation)) {
            throw new IllegalStateException(
                "expected ServerboundClientInformationPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundClientInformationPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationServerboundPackets = new ArrayList<>();
        ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationServerboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientInformation.createDefault(), ServerboundClientInformationPacket.STREAM_CODEC, ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundClientInformationPacket), ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), ServerboundClientInformationPacket.information()",
            "decompiled_source_path", "_analysis/minecraft-26.1.2/decompiled-protocol/net/minecraft/network/protocol/configuration/ConfigurationProtocols.java"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Serverbound");
        answerBody.put("packet_type", "minecraft:client_information");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("information_source", "ClientInformation.createDefault()");
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("input_information", clientInformationAnswer(information));
        answerBody.put("decoded_information", clientInformationAnswer(decodedClientInformation.information()));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_serverbound_packet_table", configurationServerboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationCookieResponseFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        Identifier key = Identifier.parse(inputFields.get("key").getAsString());
        byte[] payload = HexFormat.of().parseHex(inputFields.get("payload_hex").getAsString());
        ServerboundCookieResponsePacket packet = new ServerboundCookieResponsePacket(key, payload);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.SERVERBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ServerConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.SERVERBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ServerboundCookieResponsePacket decodedCookieResponse)) {
            throw new IllegalStateException(
                "expected ServerboundCookieResponsePacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundCookieResponsePacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationServerboundPackets = new ArrayList<>();
        ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationServerboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "Identifier.parse(String), ServerboundCookieResponsePacket(Identifier, byte[]), ServerboundCookieResponsePacket.STREAM_CODEC, ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundCookieResponsePacket), ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), ServerboundCookieResponsePacket.key(), ServerboundCookieResponsePacket.payload()",
            "decompiled_source_path", "_analysis/minecraft-26.1.2/decompiled-protocol/net/minecraft/network/protocol/cookie/ServerboundCookieResponsePacket.java"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Serverbound");
        answerBody.put("packet_type", "minecraft:cookie_response");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_key", key.toString());
        answerBody.put("decoded_key", decodedCookieResponse.key().toString());
        answerBody.put("input_payload_present", payload != null);
        answerBody.put("decoded_payload_present", decodedCookieResponse.payload() != null);
        answerBody.put("input_payload_hex", HexFormat.of().formatHex(payload));
        answerBody.put("decoded_payload_hex", HexFormat.of().formatHex(decodedCookieResponse.payload()));
        answerBody.put("input_payload_length", payload.length);
        answerBody.put("decoded_payload_length", decodedCookieResponse.payload().length);
        answerBody.put("decoded_payload_equals_input", Arrays.equals(payload, decodedCookieResponse.payload()));
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_serverbound_packet_table", configurationServerboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationCustomPayloadFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String brand = inputFields.get("brand").getAsString();
        BrandPayload payload = new BrandPayload(brand);
        ServerboundCustomPayloadPacket packet = new ServerboundCustomPayloadPacket(payload);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.SERVERBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ServerConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.SERVERBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ServerboundCustomPayloadPacket decodedCustomPayload)) {
            throw new IllegalStateException(
                "expected ServerboundCustomPayloadPacket, got " + decodedPacket.getClass().getName()
            );
        }
        if (!(decodedCustomPayload.payload() instanceof BrandPayload decodedBrandPayload)) {
            throw new IllegalStateException(
                "expected BrandPayload, got " + decodedCustomPayload.payload().getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundCustomPayloadPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        FriendlyByteBuf payloadBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        BrandPayload.STREAM_CODEC.encode(payloadBodyOut, payload);
        byte[] payloadBody = readableBytes(payloadBodyOut);

        List<Map<String, Object>> configurationServerboundPackets = new ArrayList<>();
        ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationServerboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "BrandPayload(String), BrandPayload.STREAM_CODEC, ServerboundCustomPayloadPacket(CustomPacketPayload), ServerboundCustomPayloadPacket.STREAM_CODEC, ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundCustomPayloadPacket), ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), ServerboundCustomPayloadPacket.payload(), BrandPayload.type(), BrandPayload.brand()",
            "decompiled_source_path", "_analysis/minecraft-26.1.2/decompiled-protocol/net/minecraft/network/protocol/common/ServerboundCustomPayloadPacket.java"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Serverbound");
        answerBody.put("packet_type", "minecraft:custom_payload");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_custom_payload_id", payload.type().id().toString());
        answerBody.put("decoded_custom_payload_id", decodedCustomPayload.payload().type().id().toString());
        answerBody.put("input_payload_class", payload.getClass().getName());
        answerBody.put("decoded_payload_class", decodedCustomPayload.payload().getClass().getName());
        answerBody.put("input_brand", brand);
        answerBody.put("decoded_brand", decodedBrandPayload.brand());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("encoded_payload_body_hex", HexFormat.of().formatHex(payloadBody));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_serverbound_packet_table", configurationServerboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationCustomPayloadClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String brand = inputFields.get("brand").getAsString();
        BrandPayload payload = new BrandPayload(brand);
        ClientboundCustomPayloadPacket packet = new ClientboundCustomPayloadPacket(payload);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundCustomPayloadPacket decodedCustomPayload)) {
            throw new IllegalStateException(
                "expected ClientboundCustomPayloadPacket, got " + decodedPacket.getClass().getName()
            );
        }
        if (!(decodedCustomPayload.payload() instanceof BrandPayload decodedBrandPayload)) {
            throw new IllegalStateException(
                "expected BrandPayload, got " + decodedCustomPayload.payload().getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundCustomPayloadPacket.CONFIG_STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        FriendlyByteBuf payloadBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        BrandPayload.STREAM_CODEC.encode(payloadBodyOut, payload);
        byte[] payloadBody = readableBytes(payloadBodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "BrandPayload(String), BrandPayload.STREAM_CODEC, ClientboundCustomPayloadPacket(CustomPacketPayload), ClientboundCustomPayloadPacket.CONFIG_STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundCustomPayloadPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundCustomPayloadPacket.payload(), BrandPayload.type(), BrandPayload.brand()",
            "decompiled_source_path", "_analysis/minecraft-26.1.2/decompiled-protocol/net/minecraft/network/protocol/common/ClientboundCustomPayloadPacket.java"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:custom_payload");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_custom_payload_id", payload.type().id().toString());
        answerBody.put("decoded_custom_payload_id", decodedCustomPayload.payload().type().id().toString());
        answerBody.put("input_payload_class", payload.getClass().getName());
        answerBody.put("decoded_payload_class", decodedCustomPayload.payload().getClass().getName());
        answerBody.put("input_brand", brand);
        answerBody.put("decoded_brand", decodedBrandPayload.brand());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("encoded_payload_body_hex", HexFormat.of().formatHex(payloadBody));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationDisconnectClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String reasonText = inputFields.get("reason_text").getAsString();
        Component reason = Component.literal(reasonText);
        ClientboundDisconnectPacket packet = new ClientboundDisconnectPacket(reason);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundDisconnectPacket decodedDisconnect)) {
            throw new IllegalStateException(
                "expected ClientboundDisconnectPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundDisconnectPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "Component.literal(String), ClientboundDisconnectPacket(Component), ClientboundDisconnectPacket.STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundDisconnectPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundDisconnectPacket.reason(), Component.getString()",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundDisconnectPacket net.minecraft.network.protocol.configuration.ConfigurationProtocols"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:disconnect");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("reason_fixture", "Component.literal(\"\")");
        answerBody.put("input_reason_text", reason.getString());
        answerBody.put("decoded_reason_text", decodedDisconnect.reason().getString());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationResetChatClientboundFramedDispatch(JsonObject input) {
        ClientboundResetChatPacket packet = ClientboundResetChatPacket.INSTANCE;

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundResetChatPacket decodedResetChat)) {
            throw new IllegalStateException(
                "expected ClientboundResetChatPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundResetChatPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundResetChatPacket.INSTANCE, ClientboundResetChatPacket.STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundResetChatPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundResetChatPacket.type()",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.configuration.ClientboundResetChatPacket net.minecraft.network.protocol.configuration.ConfigurationPacketTypes net.minecraft.network.protocol.configuration.ConfigurationProtocols"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:reset_chat");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("instance_packet_type", packet.type().id().toString());
        answerBody.put("decoded_equals_instance", decodedResetChat.equals(packet));
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationRegistryDataClientboundFramedDispatch(JsonObject input) {
        var registry = Registries.DIMENSION_TYPE;
        List<net.minecraft.core.RegistrySynchronization.PackedRegistryEntry> entries = List.of();
        ClientboundRegistryDataPacket packet = new ClientboundRegistryDataPacket(registry, entries);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundRegistryDataPacket decodedRegistryData)) {
            throw new IllegalStateException(
                "expected ClientboundRegistryDataPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundRegistryDataPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "Registries.DIMENSION_TYPE, ClientboundRegistryDataPacket(ResourceKey, List), ClientboundRegistryDataPacket.STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundRegistryDataPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundRegistryDataPacket.registry(), ClientboundRegistryDataPacket.entries()",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.configuration.ClientboundRegistryDataPacket 'net.minecraft.core.RegistrySynchronization$PackedRegistryEntry' net.minecraft.network.protocol.configuration.ConfigurationPacketTypes net.minecraft.network.protocol.configuration.ConfigurationProtocols net.minecraft.core.registries.Registries"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:registry_data");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("registry_fixture", "Registries.DIMENSION_TYPE with List.of() entries");
        answerBody.put("input_registry_key", registry.identifier().toString());
        answerBody.put("decoded_registry_key", decodedRegistryData.registry().identifier().toString());
        answerBody.put("input_entry_count", entries.size());
        answerBody.put("decoded_entry_count", decodedRegistryData.entries().size());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationResourcePackPopClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        UUID id = UUID.fromString(inputFields.get("id").getAsString());
        Optional<UUID> optionalId = Optional.of(id);
        ClientboundResourcePackPopPacket packet = new ClientboundResourcePackPopPacket(optionalId);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundResourcePackPopPacket decodedResourcePackPop)) {
            throw new IllegalStateException(
                "expected ClientboundResourcePackPopPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundResourcePackPopPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundResourcePackPopPacket(Optional<UUID>), ClientboundResourcePackPopPacket.STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundResourcePackPopPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundResourcePackPopPacket.id()",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundResourcePackPopPacket net.minecraft.network.protocol.configuration.ConfigurationProtocols"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:resource_pack_pop");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_uuid_present", optionalId.isPresent());
        answerBody.put("decoded_uuid_present", decodedResourcePackPop.id().isPresent());
        answerBody.put("input_uuid", id.toString());
        answerBody.put("decoded_uuid", decodedResourcePackPop.id().map(UUID::toString).orElse(""));
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationResourcePackPushClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        UUID id = UUID.fromString(inputFields.get("id").getAsString());
        String url = inputFields.get("url").getAsString();
        String hash = inputFields.get("hash").getAsString();
        boolean required = inputFields.get("required").getAsBoolean();
        Optional<Component> prompt = Optional.empty();
        ClientboundResourcePackPushPacket packet =
            new ClientboundResourcePackPushPacket(id, url, hash, required, prompt);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundResourcePackPushPacket decodedResourcePackPush)) {
            throw new IllegalStateException(
                "expected ClientboundResourcePackPushPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundResourcePackPushPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundResourcePackPushPacket(UUID, String, String, boolean, Optional<Component>), ClientboundResourcePackPushPacket.STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundResourcePackPushPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundResourcePackPushPacket.id(), url(), hash(), required(), prompt()",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundResourcePackPushPacket net.minecraft.network.protocol.configuration.ConfigurationProtocols"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:resource_pack_push");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_uuid", id.toString());
        answerBody.put("decoded_uuid", decodedResourcePackPush.id().toString());
        answerBody.put("input_url", url);
        answerBody.put("decoded_url", decodedResourcePackPush.url());
        answerBody.put("input_hash", hash);
        answerBody.put("decoded_hash", decodedResourcePackPush.hash());
        answerBody.put("input_required", required);
        answerBody.put("decoded_required", decodedResourcePackPush.required());
        answerBody.put("input_prompt_present", prompt.isPresent());
        answerBody.put("decoded_prompt_present", decodedResourcePackPush.prompt().isPresent());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationStoreCookieClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        Identifier key = Identifier.parse(inputFields.get("key").getAsString());
        byte[] payload = HexFormat.of().parseHex(inputFields.get("payload_hex").getAsString());
        ClientboundStoreCookiePacket packet = new ClientboundStoreCookiePacket(key, payload);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundStoreCookiePacket decodedStoreCookie)) {
            throw new IllegalStateException(
                "expected ClientboundStoreCookiePacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundStoreCookiePacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "Identifier.parse(String), ClientboundStoreCookiePacket(Identifier, byte[]), ClientboundStoreCookiePacket.STREAM_CODEC, ClientboundStoreCookiePacket.PAYLOAD_STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundStoreCookiePacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundStoreCookiePacket.key(), payload()",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundStoreCookiePacket net.minecraft.network.protocol.configuration.ConfigurationProtocols"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:store_cookie");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_key", key.toString());
        answerBody.put("decoded_key", decodedStoreCookie.key().toString());
        answerBody.put("input_payload_hex", HexFormat.of().formatHex(payload));
        answerBody.put("decoded_payload_hex", HexFormat.of().formatHex(decodedStoreCookie.payload()));
        answerBody.put("input_payload_length", payload.length);
        answerBody.put("decoded_payload_length", decodedStoreCookie.payload().length);
        answerBody.put("decoded_payload_equals_input", Arrays.equals(payload, decodedStoreCookie.payload()));
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationTransferClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String host = inputFields.get("host").getAsString();
        int port = inputFields.get("port").getAsInt();
        ClientboundTransferPacket packet = new ClientboundTransferPacket(host, port);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundTransferPacket decodedTransfer)) {
            throw new IllegalStateException(
                "expected ClientboundTransferPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundTransferPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundTransferPacket(String, int), ClientboundTransferPacket.STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundTransferPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundTransferPacket.host(), port()",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundTransferPacket net.minecraft.network.protocol.configuration.ConfigurationProtocols"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:transfer");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_host", host);
        answerBody.put("decoded_host", decodedTransfer.host());
        answerBody.put("input_port", port);
        answerBody.put("decoded_port", decodedTransfer.port());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationUpdateEnabledFeaturesClientboundFramedDispatch(JsonObject input) {
        Set<Identifier> features = Set.of();
        ClientboundUpdateEnabledFeaturesPacket packet =
            new ClientboundUpdateEnabledFeaturesPacket(features);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundUpdateEnabledFeaturesPacket decodedUpdateEnabledFeatures)) {
            throw new IllegalStateException(
                "expected ClientboundUpdateEnabledFeaturesPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundUpdateEnabledFeaturesPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundUpdateEnabledFeaturesPacket(Set<Identifier>), ClientboundUpdateEnabledFeaturesPacket.STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundUpdateEnabledFeaturesPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundUpdateEnabledFeaturesPacket.features()",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.configuration.ClientboundUpdateEnabledFeaturesPacket net.minecraft.network.protocol.configuration.ConfigurationProtocols net.minecraft.network.protocol.configuration.ConfigurationPacketTypes"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:update_enabled_features");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_fixture", "Set.of() features");
        answerBody.put("input_feature_count", features.size());
        answerBody.put("decoded_feature_count", decodedUpdateEnabledFeatures.features().size());
        answerBody.put("input_features", identifierStrings(features));
        answerBody.put("decoded_features", identifierStrings(decodedUpdateEnabledFeatures.features()));
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationUpdateTagsClientboundFramedDispatch(JsonObject input) {
        Map<ResourceKey<? extends Registry<?>>, TagNetworkSerialization.NetworkPayload> tags = Map.of();
        ClientboundUpdateTagsPacket packet = new ClientboundUpdateTagsPacket(tags);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundUpdateTagsPacket decodedUpdateTags)) {
            throw new IllegalStateException(
                "expected ClientboundUpdateTagsPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundUpdateTagsPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundUpdateTagsPacket(Map<ResourceKey<? extends Registry<?>>, TagNetworkSerialization.NetworkPayload>), ClientboundUpdateTagsPacket.STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundUpdateTagsPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundUpdateTagsPacket.getTags(), TagNetworkSerialization.NetworkPayload.EMPTY",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundUpdateTagsPacket net.minecraft.tags.TagNetworkSerialization net.minecraft.tags.TagNetworkSerialization\\$NetworkPayload net.minecraft.network.protocol.configuration.ConfigurationProtocols net.minecraft.network.protocol.common.CommonPacketTypes"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:update_tags");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_fixture", "Map.of() tags");
        answerBody.put("input_tag_registry_count", tags.size());
        answerBody.put("decoded_tag_registry_count", decodedUpdateTags.getTags().size());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationSelectKnownPacksClientboundFramedDispatch(JsonObject input) {
        List<KnownPack> knownPacks = List.of();
        ClientboundSelectKnownPacks packet = new ClientboundSelectKnownPacks(knownPacks);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundSelectKnownPacks decodedSelectKnownPacks)) {
            throw new IllegalStateException(
                "expected ClientboundSelectKnownPacks, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundSelectKnownPacks.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundSelectKnownPacks(List<KnownPack>), ClientboundSelectKnownPacks.STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundSelectKnownPacks), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundSelectKnownPacks.knownPacks(), KnownPack.namespace(), KnownPack.id(), KnownPack.version(), KnownPack.isVanilla()",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.configuration.ClientboundSelectKnownPacks net.minecraft.network.protocol.configuration.ConfigurationProtocols net.minecraft.network.protocol.configuration.ConfigurationPacketTypes net.minecraft.server.packs.repository.KnownPack"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:select_known_packs");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_fixture", "List.of() known_packs");
        answerBody.put("input_known_packs", knownPackAnswers(knownPacks));
        answerBody.put("decoded_known_packs", knownPackAnswers(decodedSelectKnownPacks.knownPacks()));
        answerBody.put("input_known_pack_count", knownPacks.size());
        answerBody.put("decoded_known_pack_count", decodedSelectKnownPacks.knownPacks().size());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationCustomReportDetailsClientboundFramedDispatch(JsonObject input) {
        Map<String, String> details = Map.of();
        ClientboundCustomReportDetailsPacket packet = new ClientboundCustomReportDetailsPacket(details);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundCustomReportDetailsPacket decodedCustomReportDetails)) {
            throw new IllegalStateException(
                "expected ClientboundCustomReportDetailsPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundCustomReportDetailsPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundCustomReportDetailsPacket(Map<String, String>), ClientboundCustomReportDetailsPacket.STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundCustomReportDetailsPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundCustomReportDetailsPacket.details()",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundCustomReportDetailsPacket net.minecraft.network.protocol.common.CommonPacketTypes net.minecraft.network.protocol.configuration.ConfigurationProtocols"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:custom_report_details");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_fixture", "Map.of() details");
        answerBody.put("input_details", details);
        answerBody.put("decoded_details", decodedCustomReportDetails.details());
        answerBody.put("input_detail_count", details.size());
        answerBody.put("decoded_detail_count", decodedCustomReportDetails.details().size());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationServerLinksClientboundFramedDispatch(JsonObject input) {
        List<ServerLinks.UntrustedEntry> links = List.of();
        ClientboundServerLinksPacket packet = new ClientboundServerLinksPacket(links);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundServerLinksPacket decodedServerLinks)) {
            throw new IllegalStateException(
                "expected ClientboundServerLinksPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundServerLinksPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundServerLinksPacket(List<ServerLinks.UntrustedEntry>), ClientboundServerLinksPacket.STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundServerLinksPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundServerLinksPacket.links(), ServerLinks.UNTRUSTED_LINKS_STREAM_CODEC",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundServerLinksPacket net.minecraft.server.ServerLinks net.minecraft.server.ServerLinks\\$UntrustedEntry net.minecraft.server.ServerLinks\\$KnownLinkType net.minecraft.network.protocol.common.CommonPacketTypes net.minecraft.network.protocol.configuration.ConfigurationProtocols"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:server_links");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_fixture", "List.of() server_links");
        answerBody.put("input_links", serverLinkAnswers(links));
        answerBody.put("decoded_links", serverLinkAnswers(decodedServerLinks.links()));
        answerBody.put("input_link_count", links.size());
        answerBody.put("decoded_link_count", decodedServerLinks.links().size());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationClearDialogClientboundFramedDispatch(JsonObject input) {
        ClientboundClearDialogPacket packet = ClientboundClearDialogPacket.INSTANCE;

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundClearDialogPacket decodedClearDialog)) {
            throw new IllegalStateException(
                "expected ClientboundClearDialogPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundClearDialogPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundClearDialogPacket.INSTANCE, ClientboundClearDialogPacket.STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundClearDialogPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundClearDialogPacket.type()",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundClearDialogPacket net.minecraft.network.protocol.common.CommonPacketTypes net.minecraft.network.protocol.configuration.ConfigurationProtocols"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:clear_dialog");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("instance_packet_type", packet.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_fixture", "ClientboundClearDialogPacket.INSTANCE");
        answerBody.put("decoded_equals_instance", decodedClearDialog == ClientboundClearDialogPacket.INSTANCE);
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationShowDialogClientboundFramedDispatch(JsonObject input) {
        CommonDialogData common = new CommonDialogData(
            Component.literal("Oracle notice"),
            Optional.empty(),
            true,
            false,
            DialogAction.CLOSE,
            List.of(),
            List.of()
        );
        Dialog dialog = new NoticeDialog(common, NoticeDialog.DEFAULT_ACTION);
        ClientboundShowDialogPacket packet = new ClientboundShowDialogPacket(Holder.direct(dialog));

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundShowDialogPacket decodedShowDialog)) {
            throw new IllegalStateException(
                "expected ClientboundShowDialogPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundShowDialogPacket.CONTEXT_FREE_STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);
        Dialog decodedDialog = decodedShowDialog.dialog().value();

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundShowDialogPacket(Holder.direct(NoticeDialog)), ClientboundShowDialogPacket.CONTEXT_FREE_STREAM_CODEC, Dialog.CONTEXT_FREE_STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundShowDialogPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundShowDialogPacket.dialog(), NoticeDialog.DEFAULT_ACTION",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundShowDialogPacket net.minecraft.server.dialog.Dialog net.minecraft.server.dialog.NoticeDialog net.minecraft.server.dialog.CommonDialogData net.minecraft.server.dialog.ActionButton net.minecraft.server.dialog.CommonButtonData net.minecraft.network.protocol.common.CommonPacketTypes net.minecraft.network.protocol.configuration.ConfigurationProtocols"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:show_dialog");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_fixture", "Holder.direct(new NoticeDialog(CommonDialogData literal title, NoticeDialog.DEFAULT_ACTION))");
        answerBody.put("input_dialog_class", dialog.getClass().getName());
        answerBody.put("decoded_dialog_class", decodedDialog.getClass().getName());
        answerBody.put("input_dialog_title", common.title().getString());
        answerBody.put("decoded_dialog_title", decodedDialog.common().title().getString());
        answerBody.put("input_dialog_body_count", common.body().size());
        answerBody.put("decoded_dialog_body_count", decodedDialog.common().body().size());
        answerBody.put("input_dialog_input_count", common.inputs().size());
        answerBody.put("decoded_dialog_input_count", decodedDialog.common().inputs().size());
        answerBody.put("input_can_close_with_escape", common.canCloseWithEscape());
        answerBody.put("decoded_can_close_with_escape", decodedDialog.common().canCloseWithEscape());
        answerBody.put("input_pause", common.pause());
        answerBody.put("decoded_pause", decodedDialog.common().pause());
        answerBody.put("input_after_action", common.afterAction().getSerializedName());
        answerBody.put("decoded_after_action", decodedDialog.common().afterAction().getSerializedName());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationCodeOfConductClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String codeOfConduct = inputFields.get("code_of_conduct").getAsString();
        ClientboundCodeOfConductPacket packet = new ClientboundCodeOfConductPacket(codeOfConduct);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundCodeOfConductPacket decodedCodeOfConduct)) {
            throw new IllegalStateException(
                "expected ClientboundCodeOfConductPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundCodeOfConductPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundCodeOfConductPacket(String), ClientboundCodeOfConductPacket.STREAM_CODEC, ByteBufCodecs.STRING_UTF8, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundCodeOfConductPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundCodeOfConductPacket.codeOfConduct()",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.configuration.ClientboundCodeOfConductPacket net.minecraft.network.protocol.configuration.ConfigurationPacketTypes net.minecraft.network.protocol.configuration.ConfigurationProtocols"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:code_of_conduct");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_fixture", "ClientboundCodeOfConductPacket(String)");
        answerBody.put("input_code_of_conduct", codeOfConduct);
        answerBody.put("decoded_code_of_conduct", decodedCodeOfConduct.codeOfConduct());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationResourcePackResponseFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        UUID id = UUID.fromString(inputFields.get("id").getAsString());
        ServerboundResourcePackPacket.Action action =
            ServerboundResourcePackPacket.Action.valueOf(inputFields.get("action").getAsString());
        ServerboundResourcePackPacket packet = new ServerboundResourcePackPacket(id, action);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.SERVERBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ServerConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.SERVERBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ServerboundResourcePackPacket decodedResourcePack)) {
            throw new IllegalStateException(
                "expected ServerboundResourcePackPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundResourcePackPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationServerboundPackets = new ArrayList<>();
        ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationServerboundPackets.add(row);
        });

        List<Map<String, Object>> actionRows = new ArrayList<>();
        for (ServerboundResourcePackPacket.Action enumAction : ServerboundResourcePackPacket.Action.values()) {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("name", enumAction.name());
            row.put("is_terminal", enumAction.isTerminal());
            actionRows.add(row);
        }

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ServerboundResourcePackPacket(UUID, Action), ServerboundResourcePackPacket.STREAM_CODEC, ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundResourcePackPacket), ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), ServerboundResourcePackPacket.id(), ServerboundResourcePackPacket.action(), ServerboundResourcePackPacket.Action.isTerminal()",
            "decompiled_source_path", "_analysis/minecraft-26.1.2/decompiled-protocol/net/minecraft/network/protocol/common/ServerboundResourcePackPacket.java"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Serverbound");
        answerBody.put("packet_type", "minecraft:resource_pack");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_uuid", id.toString());
        answerBody.put("decoded_uuid", decodedResourcePack.id().toString());
        answerBody.put("input_action", action.name());
        answerBody.put("decoded_action", decodedResourcePack.action().name());
        answerBody.put("input_action_is_terminal", action.isTerminal());
        answerBody.put("decoded_action_is_terminal", decodedResourcePack.action().isTerminal());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_serverbound_packet_table", configurationServerboundPackets);
        answerBody.put("resource_pack_action_table", actionRows);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> loginAcknowledgedServerboundFramedDispatch(JsonObject input) {
        ServerboundLoginAcknowledgedPacket packet = ServerboundLoginAcknowledgedPacket.INSTANCE;

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        LoginProtocols.SERVERBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ServerLoginPacketListener> decodedPacket =
            LoginProtocols.SERVERBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ServerboundLoginAcknowledgedPacket decodedLoginAcknowledged)) {
            throw new IllegalStateException(
                "expected ServerboundLoginAcknowledgedPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundLoginAcknowledgedPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> loginServerboundPackets = new ArrayList<>();
        LoginProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            loginServerboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ServerboundLoginAcknowledgedPacket.INSTANCE, ServerboundLoginAcknowledgedPacket.STREAM_CODEC, LoginProtocols.SERVERBOUND.codec().encode/decode(ServerboundLoginAcknowledgedPacket), LoginProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), ServerboundLoginAcknowledgedPacket.type(), ServerboundLoginAcknowledgedPacket.isTerminal()",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ServerboundLoginAcknowledgedPacket"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Login");
        answerBody.put("flow", "Serverbound");
        answerBody.put("packet_type", "minecraft:login_acknowledged");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_fixture", "ServerboundLoginAcknowledgedPacket.INSTANCE");
        answerBody.put("instance_packet_type", packet.type().id().toString());
        answerBody.put("decoded_equals_instance", decodedLoginAcknowledged == ServerboundLoginAcknowledgedPacket.INSTANCE);
        answerBody.put("input_is_terminal", packet.isTerminal());
        answerBody.put("decoded_is_terminal", decodedLoginAcknowledged.isTerminal());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("login_serverbound_packet_table", loginServerboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> loginCookieResponseServerboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        Identifier key = Identifier.parse(inputFields.get("key").getAsString());
        byte[] payload = HexFormat.of().parseHex(inputFields.get("payload_hex").getAsString());
        ServerboundCookieResponsePacket packet = new ServerboundCookieResponsePacket(key, payload);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        LoginProtocols.SERVERBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ServerLoginPacketListener> decodedPacket =
            LoginProtocols.SERVERBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ServerboundCookieResponsePacket decodedCookieResponse)) {
            throw new IllegalStateException(
                "expected ServerboundCookieResponsePacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundCookieResponsePacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> loginServerboundPackets = new ArrayList<>();
        LoginProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            loginServerboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "Identifier.parse(String), ServerboundCookieResponsePacket(Identifier, byte[]), ServerboundCookieResponsePacket.STREAM_CODEC, LoginProtocols.SERVERBOUND.codec().encode/decode(ServerboundCookieResponsePacket), LoginProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), ServerboundCookieResponsePacket.key(), ServerboundCookieResponsePacket.payload(), ServerLoginPacketListener extends ServerCookiePacketListener",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.cookie.ServerboundCookieResponsePacket"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Login");
        answerBody.put("flow", "Serverbound");
        answerBody.put("packet_type", "minecraft:cookie_response");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_key", key.toString());
        answerBody.put("decoded_key", decodedCookieResponse.key().toString());
        answerBody.put("input_payload_present", payload != null);
        answerBody.put("decoded_payload_present", decodedCookieResponse.payload() != null);
        answerBody.put("input_payload_hex", HexFormat.of().formatHex(payload));
        answerBody.put("decoded_payload_hex", HexFormat.of().formatHex(decodedCookieResponse.payload()));
        answerBody.put("input_payload_length", payload.length);
        answerBody.put("decoded_payload_length", decodedCookieResponse.payload().length);
        answerBody.put("decoded_payload_equals_input", Arrays.equals(payload, decodedCookieResponse.payload()));
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("login_serverbound_packet_table", loginServerboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> loginCookieRequestClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        Identifier key = Identifier.parse(inputFields.get("key").getAsString());
        ClientboundCookieRequestPacket packet = new ClientboundCookieRequestPacket(key);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        LoginProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientLoginPacketListener> decodedPacket =
            LoginProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundCookieRequestPacket decodedCookieRequest)) {
            throw new IllegalStateException(
                "expected ClientboundCookieRequestPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundCookieRequestPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> loginClientboundPackets = new ArrayList<>();
        LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            loginClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "Identifier.parse(String), ClientboundCookieRequestPacket(Identifier), ClientboundCookieRequestPacket.STREAM_CODEC, FriendlyByteBuf.readIdentifier/writeIdentifier, LoginProtocols.CLIENTBOUND.codec().encode/decode(ClientboundCookieRequestPacket), LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundCookieRequestPacket.key(), ClientLoginPacketListener extends ClientCookiePacketListener",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.cookie.ClientboundCookieRequestPacket net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ClientLoginPacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Login");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:cookie_request");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_key", key.toString());
        answerBody.put("decoded_key", decodedCookieRequest.key().toString());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("login_clientbound_packet_table", loginClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> playBundleDelimiterClientboundFramedDispatch(JsonObject input) {
        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] bundleDelimiterPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:bundle_delimiter".equals(type.id().toString())) {
                bundleDelimiterPacketId[0] = packetId;
            }
        });
        if (bundleDelimiterPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound bundle_delimiter packet id");
        }

        var protocolInfo = GameProtocols.CLIENTBOUND_TEMPLATE.bind(
            RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)
        );
        FriendlyByteBuf seedOut = new FriendlyByteBuf(Unpooled.buffer());
        seedOut.writeVarInt(bundleDelimiterPacketId[0]);
        byte[] seedFrame = readableBytes(seedOut);

        RegistryFriendlyByteBuf framedIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(seedFrame), RegistryAccess.EMPTY);
        Packet<? super ClientGamePacketListener> decodedPacket =
            protocolInfo.codec().decode(framedIn);

        RegistryFriendlyByteBuf framedOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), RegistryAccess.EMPTY);
        protocolInfo.codec().encode(framedOut, decodedPacket);
        byte[] framed = readableBytes(framedOut);
        byte[] body = bytesAfterVarIntPrefix(framed);

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().decode/encode(...), ClientboundBundleDelimiterPacket.type(), ClientGamePacketListener",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.network.protocol.BundleDelimiterPacket net.minecraft.network.protocol.BundlerInfo net.minecraft.network.ProtocolInfo"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:bundle_delimiter");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_fixture", "official Play clientbound bundle_delimiter table id decoded to the registered delimiter singleton");
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("play_clientbound_packet_table", playClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> playAddEntityClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int entityId = inputFields.get("entity_id").getAsInt();
        UUID uuid = UUID.fromString(inputFields.get("uuid").getAsString());
        double x = inputFields.get("x").getAsDouble();
        double y = inputFields.get("y").getAsDouble();
        double z = inputFields.get("z").getAsDouble();
        float xRot = inputFields.get("x_rot_degrees").getAsFloat();
        float yRot = inputFields.get("y_rot_degrees").getAsFloat();
        double yHeadRot = inputFields.get("y_head_rot_degrees").getAsDouble();
        int data = inputFields.get("data").getAsInt();
        Vec3 movement = Vec3.ZERO;

        ClientboundAddEntityPacket packet = new ClientboundAddEntityPacket(
            entityId,
            uuid,
            x,
            y,
            z,
            xRot,
            yRot,
            EntityType.PIG,
            data,
            movement,
            yHeadRot
        );

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] addEntityPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:add_entity".equals(type.id().toString())) {
                addEntityPacketId[0] = packetId;
            }
        });
        if (addEntityPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound add_entity packet id");
        }

        RegistryAccess.Frozen registryAccess =
            RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY);
        var protocolInfo = GameProtocols.CLIENTBOUND_TEMPLATE.bind(
            RegistryFriendlyByteBuf.decorator(registryAccess)
        );
        RegistryFriendlyByteBuf framedOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        protocolInfo.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);
        byte[] body = bytesAfterVarIntPrefix(framed);

        RegistryFriendlyByteBuf framedIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(framed), registryAccess);
        Packet<? super ClientGamePacketListener> decodedPacket =
            protocolInfo.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundAddEntityPacket decodedAddEntity)) {
            throw new IllegalStateException(
                "decoded Play add_entity as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf movementOut = new FriendlyByteBuf(Unpooled.buffer());
        Vec3.LP_STREAM_CODEC.encode(movementOut, movement);
        byte[] movementBody = readableBytes(movementOut);

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundAddEntityPacket(int, UUID, double, double, double, float, float, EntityType<?>, int, Vec3, double), ClientboundAddEntityPacket.STREAM_CODEC, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY))).codec().encode/decode(...), EntityType.PIG, BuiltInRegistries.ENTITY_TYPE, Vec3.LP_STREAM_CODEC, ClientGamePacketListener.handleAddEntity(ClientboundAddEntityPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundAddEntityPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.core.RegistryAccess net.minecraft.core.registries.BuiltInRegistries net.minecraft.world.entity.EntityType net.minecraft.world.phys.Vec3"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:add_entity");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "direct official ClientboundAddEntityPacket constructor with bootstrapped built-in EntityType.PIG and zero Vec3.LP movement");
        answerBody.put("input_entity_id", entityId);
        answerBody.put("decoded_entity_id", decodedAddEntity.getId());
        answerBody.put("input_uuid", uuid.toString());
        answerBody.put("decoded_uuid", decodedAddEntity.getUUID().toString());
        answerBody.put("input_entity_type", "minecraft:pig");
        answerBody.put("decoded_entity_type", BuiltInRegistries.ENTITY_TYPE.getKey(decodedAddEntity.getType()).toString());
        answerBody.put("decoded_entity_type_registry_id", BuiltInRegistries.ENTITY_TYPE.getId(decodedAddEntity.getType()));
        answerBody.put("input_x", x);
        answerBody.put("decoded_x", decodedAddEntity.getX());
        answerBody.put("input_y", y);
        answerBody.put("decoded_y", decodedAddEntity.getY());
        answerBody.put("input_z", z);
        answerBody.put("decoded_z", decodedAddEntity.getZ());
        answerBody.put("input_movement_x", movement.x());
        answerBody.put("decoded_movement_x", decodedAddEntity.getMovement().x());
        answerBody.put("input_movement_y", movement.y());
        answerBody.put("decoded_movement_y", decodedAddEntity.getMovement().y());
        answerBody.put("input_movement_z", movement.z());
        answerBody.put("decoded_movement_z", decodedAddEntity.getMovement().z());
        answerBody.put("encoded_movement_lp_hex", HexFormat.of().formatHex(movementBody));
        answerBody.put("input_x_rot_degrees", xRot);
        answerBody.put("decoded_x_rot_degrees", decodedAddEntity.getXRot());
        answerBody.put("decoded_x_rot_byte", privateByte(decodedAddEntity, "xRot"));
        answerBody.put("input_y_rot_degrees", yRot);
        answerBody.put("decoded_y_rot_degrees", decodedAddEntity.getYRot());
        answerBody.put("decoded_y_rot_byte", privateByte(decodedAddEntity, "yRot"));
        answerBody.put("input_y_head_rot_degrees", yHeadRot);
        answerBody.put("decoded_y_head_rot_degrees", decodedAddEntity.getYHeadRot());
        answerBody.put("decoded_y_head_rot_byte", privateByte(decodedAddEntity, "yHeadRot"));
        answerBody.put("input_data", data);
        answerBody.put("decoded_data", decodedAddEntity.getData());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("play_clientbound_packet_table", playClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> playAnimateClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int entityId = inputFields.get("entity_id").getAsInt();
        int action = ClientboundAnimatePacket.SWING_MAIN_HAND;

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        fixtureBodyOut.writeVarInt(entityId);
        fixtureBodyOut.writeByte(action);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundAnimatePacket packet = ClientboundAnimatePacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] animatePacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:animate".equals(type.id().toString())) {
                animatePacketId[0] = packetId;
            }
        });
        if (animatePacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound animate packet id");
        }

        var protocolInfo = GameProtocols.CLIENTBOUND_TEMPLATE.bind(
            RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)
        );
        RegistryFriendlyByteBuf framedOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), RegistryAccess.EMPTY);
        protocolInfo.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);
        byte[] body = bytesAfterVarIntPrefix(framed);

        RegistryFriendlyByteBuf framedIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(framed), RegistryAccess.EMPTY);
        Packet<? super ClientGamePacketListener> decodedPacket =
            protocolInfo.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundAnimatePacket decodedAnimate)) {
            throw new IllegalStateException(
                "decoded Play animate as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        List<Map<String, Object>> actionConstants = new ArrayList<>();
        actionConstants.add(Map.of("name", "SWING_MAIN_HAND", "value", ClientboundAnimatePacket.SWING_MAIN_HAND));
        actionConstants.add(Map.of("name", "WAKE_UP", "value", ClientboundAnimatePacket.WAKE_UP));
        actionConstants.add(Map.of("name", "SWING_OFF_HAND", "value", ClientboundAnimatePacket.SWING_OFF_HAND));
        actionConstants.add(Map.of("name", "CRITICAL_HIT", "value", ClientboundAnimatePacket.CRITICAL_HIT));
        actionConstants.add(Map.of("name", "MAGIC_CRITICAL_HIT", "value", ClientboundAnimatePacket.MAGIC_CRITICAL_HIT));

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundAnimatePacket.STREAM_CODEC, private ClientboundAnimatePacket(FriendlyByteBuf), private write(FriendlyByteBuf), ClientboundAnimatePacket.SWING_MAIN_HAND, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...), ClientGamePacketListener.handleAnimate(ClientboundAnimatePacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p -verbose net.minecraft.network.protocol.game.ClientboundAnimatePacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:animate");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundAnimatePacket.STREAM_CODEC decode fixture with entity id and SWING_MAIN_HAND action; no initialized Entity, Level, or game state");
        answerBody.put("input_entity_id", entityId);
        answerBody.put("decoded_entity_id", decodedAnimate.getId());
        answerBody.put("input_animation_action_name", "SWING_MAIN_HAND");
        answerBody.put("input_animation_action", action);
        answerBody.put("decoded_animation_action", decodedAnimate.getAction());
        answerBody.put("remaining_after_packet_stream_decode", packetIn.readableBytes());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("fixture_body_hex", HexFormat.of().formatHex(fixtureBody));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("animate_action_constants", actionConstants);
        answerBody.put("play_clientbound_packet_table", playClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> playAwardStatsClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int expectedStatCount = inputFields.get("stat_count").getAsInt();
        Object2IntMap<Stat<?>> stats = new Object2IntOpenHashMap<>();
        if (expectedStatCount != stats.size()) {
            throw new IllegalArgumentException("minimal award_stats fixture only supports empty stats");
        }
        ClientboundAwardStatsPacket packet = new ClientboundAwardStatsPacket(stats);

        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), RegistryAccess.EMPTY);
        ClientboundAwardStatsPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), RegistryAccess.EMPTY);
        ClientboundAwardStatsPacket streamDecoded =
            ClientboundAwardStatsPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] awardStatsPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:award_stats".equals(type.id().toString())) {
                awardStatsPacketId[0] = packetId;
            }
        });
        if (awardStatsPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound award_stats packet id");
        }

        var protocolInfo = GameProtocols.CLIENTBOUND_TEMPLATE.bind(
            RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)
        );
        RegistryFriendlyByteBuf framedOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), RegistryAccess.EMPTY);
        protocolInfo.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);
        byte[] body = bytesAfterVarIntPrefix(framed);

        RegistryFriendlyByteBuf framedIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(framed), RegistryAccess.EMPTY);
        Packet<? super ClientGamePacketListener> decodedPacket =
            protocolInfo.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundAwardStatsPacket decodedAwardStats)) {
            throw new IllegalStateException(
                "decoded Play award_stats as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundAwardStatsPacket(Object2IntMap<Stat<?>>), ClientboundAwardStatsPacket.STREAM_CODEC, ClientboundAwardStatsPacket.STAT_VALUES_STREAM_CODEC, Stat.STREAM_CODEC, ByteBufCodecs.VAR_INT, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...), ClientGamePacketListener.handleAwardStats(ClientboundAwardStatsPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p -verbose net.minecraft.network.protocol.game.ClientboundAwardStatsPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.stats.Stat net.minecraft.stats.StatType"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:award_stats");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundAwardStatsPacket empty Object2IntOpenHashMap<Stat<?>> fixture; no initialized Minecraft/game state or stat registry entries");
        answerBody.put("official_body_shape", "Object2IntMap<Stat<?>> encoded as VarInt count, then per entry Stat.STREAM_CODEC key followed by VarInt value; empty fixture encodes only count 0");
        answerBody.put("input_stat_count", stats.size());
        answerBody.put("stream_decoded_stat_count", streamDecoded.stats().size());
        answerBody.put("decoded_stat_count", decodedAwardStats.stats().size());
        answerBody.put("remaining_after_packet_stream_decode", packetIn.readableBytes());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("fixture_body_hex", HexFormat.of().formatHex(fixtureBody));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("play_clientbound_packet_table", playClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> playBlockChangedAckClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int sequence = inputFields.get("sequence").getAsInt();
        ClientboundBlockChangedAckPacket packet = new ClientboundBlockChangedAckPacket(sequence);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundBlockChangedAckPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundBlockChangedAckPacket streamDecoded =
            ClientboundBlockChangedAckPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] blockChangedAckPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:block_changed_ack".equals(type.id().toString())) {
                blockChangedAckPacketId[0] = packetId;
            }
        });
        if (blockChangedAckPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound block_changed_ack packet id");
        }

        var protocolInfo = GameProtocols.CLIENTBOUND_TEMPLATE.bind(
            RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)
        );
        RegistryFriendlyByteBuf framedOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), RegistryAccess.EMPTY);
        protocolInfo.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);
        byte[] body = bytesAfterVarIntPrefix(framed);

        RegistryFriendlyByteBuf framedIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(framed), RegistryAccess.EMPTY);
        Packet<? super ClientGamePacketListener> decodedPacket =
            protocolInfo.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundBlockChangedAckPacket decodedBlockChangedAck)) {
            throw new IllegalStateException(
                "decoded Play block_changed_ack as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundBlockChangedAckPacket(int), ClientboundBlockChangedAckPacket.STREAM_CODEC, private ClientboundBlockChangedAckPacket(FriendlyByteBuf), private write(FriendlyByteBuf), FriendlyByteBuf.readVarInt/writeVarInt, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...), ClientGamePacketListener.handleBlockChangedAck(ClientboundBlockChangedAckPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p -verbose net.minecraft.network.protocol.game.ClientboundBlockChangedAckPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:block_changed_ack");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundBlockChangedAckPacket sequence constructor fixture; no initialized Minecraft/game state");
        answerBody.put("official_body_shape", "sequence encoded as one FriendlyByteBuf VarInt");
        answerBody.put("input_sequence", sequence);
        answerBody.put("stream_decoded_sequence", streamDecoded.sequence());
        answerBody.put("decoded_sequence", decodedBlockChangedAck.sequence());
        answerBody.put("remaining_after_packet_stream_decode", packetIn.readableBytes());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("fixture_body_hex", HexFormat.of().formatHex(fixtureBody));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("play_clientbound_packet_table", playClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> playBlockDestructionClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int breakerId = inputFields.get("breaker_id").getAsInt();
        int blockX = inputFields.get("block_x").getAsInt();
        int blockY = inputFields.get("block_y").getAsInt();
        int blockZ = inputFields.get("block_z").getAsInt();
        int progress = inputFields.get("progress").getAsInt();
        BlockPos pos = new BlockPos(blockX, blockY, blockZ);
        ClientboundBlockDestructionPacket packet =
            new ClientboundBlockDestructionPacket(breakerId, pos, progress);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundBlockDestructionPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundBlockDestructionPacket streamDecoded =
            ClientboundBlockDestructionPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] blockDestructionPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:block_destruction".equals(type.id().toString())) {
                blockDestructionPacketId[0] = packetId;
            }
        });
        if (blockDestructionPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound block_destruction packet id");
        }

        var protocolInfo = GameProtocols.CLIENTBOUND_TEMPLATE.bind(
            RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)
        );
        RegistryFriendlyByteBuf framedOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), RegistryAccess.EMPTY);
        protocolInfo.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);
        byte[] body = bytesAfterVarIntPrefix(framed);

        RegistryFriendlyByteBuf framedIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(framed), RegistryAccess.EMPTY);
        Packet<? super ClientGamePacketListener> decodedPacket =
            protocolInfo.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundBlockDestructionPacket decodedBlockDestruction)) {
            throw new IllegalStateException(
                "decoded Play block_destruction as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundBlockDestructionPacket(int, BlockPos, int), ClientboundBlockDestructionPacket.STREAM_CODEC, private ClientboundBlockDestructionPacket(FriendlyByteBuf), private write(FriendlyByteBuf), FriendlyByteBuf.readVarInt/writeVarInt, FriendlyByteBuf.readBlockPos/writeBlockPos, FriendlyByteBuf.readUnsignedByte/writeByte, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...), ClientGamePacketListener.handleBlockDestruction(ClientboundBlockDestructionPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p -verbose net.minecraft.network.protocol.game.ClientboundBlockDestructionPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener"
        ));

        BlockPos streamDecodedPos = streamDecoded.getPos();
        BlockPos decodedPos = decodedBlockDestruction.getPos();
        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:block_destruction");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundBlockDestructionPacket breaker id, BlockPos, and progress constructor fixture; no initialized Minecraft/game state");
        answerBody.put("official_body_shape", "breaker id encoded as FriendlyByteBuf VarInt, block position encoded with FriendlyByteBuf BlockPos, progress encoded as one unsigned byte");
        answerBody.put("input_breaker_id", breakerId);
        answerBody.put("stream_decoded_breaker_id", streamDecoded.getId());
        answerBody.put("decoded_breaker_id", decodedBlockDestruction.getId());
        answerBody.put("input_block_x", blockX);
        answerBody.put("input_block_y", blockY);
        answerBody.put("input_block_z", blockZ);
        answerBody.put("stream_decoded_block_x", streamDecodedPos.getX());
        answerBody.put("stream_decoded_block_y", streamDecodedPos.getY());
        answerBody.put("stream_decoded_block_z", streamDecodedPos.getZ());
        answerBody.put("decoded_block_x", decodedPos.getX());
        answerBody.put("decoded_block_y", decodedPos.getY());
        answerBody.put("decoded_block_z", decodedPos.getZ());
        answerBody.put("input_progress", progress);
        answerBody.put("stream_decoded_progress", streamDecoded.getProgress());
        answerBody.put("decoded_progress", decodedBlockDestruction.getProgress());
        answerBody.put("remaining_after_packet_stream_decode", packetIn.readableBytes());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("fixture_body_hex", HexFormat.of().formatHex(fixtureBody));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("play_clientbound_packet_table", playClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> playBlockEntityDataClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int blockX = inputFields.get("block_x").getAsInt();
        int blockY = inputFields.get("block_y").getAsInt();
        int blockZ = inputFields.get("block_z").getAsInt();
        String expectedBlockEntityType = inputFields.get("block_entity_type").getAsString();
        int expectedTagSize = inputFields.get("tag_size").getAsInt();
        BlockEntityType<?> type = BlockEntityType.CHEST;
        String blockEntityType = BuiltInRegistries.BLOCK_ENTITY_TYPE.getKey(type).toString();
        if (!expectedBlockEntityType.equals(blockEntityType)) {
            throw new IllegalArgumentException(
                "minimal block_entity_data fixture expected " + expectedBlockEntityType
                    + " but official type is " + blockEntityType
            );
        }
        BlockPos pos = new BlockPos(blockX, blockY, blockZ);
        CompoundTag tag = new CompoundTag();
        if (expectedTagSize != tag.size()) {
            throw new IllegalArgumentException("minimal block_entity_data fixture only supports empty tag");
        }
        ClientboundBlockEntityDataPacket packet =
            constructBlockEntityDataPacket(pos, type, tag);
        RegistryAccess registryAccess =
            RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY);

        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundBlockEntityDataPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundBlockEntityDataPacket streamDecoded =
            ClientboundBlockEntityDataPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] blockEntityDataPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((packetType, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", packetType.id().toString());
            row.put("flow", packetType.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:block_entity_data".equals(packetType.id().toString())) {
                blockEntityDataPacketId[0] = packetId;
            }
        });
        if (blockEntityDataPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound block_entity_data packet id");
        }

        var protocolInfo = GameProtocols.CLIENTBOUND_TEMPLATE.bind(
            RegistryFriendlyByteBuf.decorator(registryAccess)
        );
        RegistryFriendlyByteBuf framedOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        protocolInfo.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);
        byte[] body = bytesAfterVarIntPrefix(framed);

        RegistryFriendlyByteBuf framedIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(framed), registryAccess);
        Packet<? super ClientGamePacketListener> decodedPacket =
            protocolInfo.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundBlockEntityDataPacket decodedBlockEntityData)) {
            throw new IllegalStateException(
                "decoded Play block_entity_data as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        BlockPos streamDecodedPos = streamDecoded.getPos();
        BlockPos decodedPos = decodedBlockEntityData.getPos();
        String streamDecodedType = BuiltInRegistries.BLOCK_ENTITY_TYPE
            .getKey(streamDecoded.getType())
            .toString();
        String decodedType = BuiltInRegistries.BLOCK_ENTITY_TYPE
            .getKey(decodedBlockEntityData.getType())
            .toString();
        int typeRegistryId = BuiltInRegistries.BLOCK_ENTITY_TYPE.getId(type);

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "private ClientboundBlockEntityDataPacket(BlockPos, BlockEntityType<?>, CompoundTag), ClientboundBlockEntityDataPacket.STREAM_CODEC, BlockPos.STREAM_CODEC, ByteBufCodecs.registry(Registries.BLOCK_ENTITY_TYPE), ByteBufCodecs.TRUSTED_COMPOUND_TAG, BuiltInRegistries.BLOCK_ENTITY_TYPE, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY))).codec().encode/decode(...), ClientGamePacketListener.handleBlockEntityData(ClientboundBlockEntityDataPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p -verbose net.minecraft.network.protocol.game.ClientboundBlockEntityDataPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.world.level.block.entity.BlockEntityType net.minecraft.core.registries.BuiltInRegistries net.minecraft.nbt.CompoundTag"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:block_entity_data");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official private ClientboundBlockEntityDataPacket BlockPos, built-in BlockEntityType.CHEST, and empty CompoundTag constructor fixture; requires bootstrapped built-in registries but no initialized Level, BlockEntity, or game state");
        answerBody.put("official_body_shape", "block position encoded with BlockPos.STREAM_CODEC, block entity type encoded with ByteBufCodecs.registry(Registries.BLOCK_ENTITY_TYPE), and tag encoded with ByteBufCodecs.TRUSTED_COMPOUND_TAG");
        answerBody.put("input_block_x", blockX);
        answerBody.put("input_block_y", blockY);
        answerBody.put("input_block_z", blockZ);
        answerBody.put("stream_decoded_block_x", streamDecodedPos.getX());
        answerBody.put("stream_decoded_block_y", streamDecodedPos.getY());
        answerBody.put("stream_decoded_block_z", streamDecodedPos.getZ());
        answerBody.put("decoded_block_x", decodedPos.getX());
        answerBody.put("decoded_block_y", decodedPos.getY());
        answerBody.put("decoded_block_z", decodedPos.getZ());
        answerBody.put("input_block_entity_type", blockEntityType);
        answerBody.put("stream_decoded_block_entity_type", streamDecodedType);
        answerBody.put("decoded_block_entity_type", decodedType);
        answerBody.put("decoded_block_entity_type_registry_id", typeRegistryId);
        answerBody.put("input_tag_size", tag.size());
        answerBody.put("stream_decoded_tag_size", streamDecoded.getTag().size());
        answerBody.put("decoded_tag_size", decodedBlockEntityData.getTag().size());
        answerBody.put("input_tag_snbt", tag.toString());
        answerBody.put("stream_decoded_tag_snbt", streamDecoded.getTag().toString());
        answerBody.put("decoded_tag_snbt", decodedBlockEntityData.getTag().toString());
        answerBody.put("remaining_after_packet_stream_decode", packetIn.readableBytes());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("fixture_body_hex", HexFormat.of().formatHex(fixtureBody));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("play_clientbound_packet_table", playClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> playBlockEventClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int blockX = inputFields.get("block_x").getAsInt();
        int blockY = inputFields.get("block_y").getAsInt();
        int blockZ = inputFields.get("block_z").getAsInt();
        String expectedBlock = inputFields.get("block").getAsString();
        int eventType = inputFields.get("event_type").getAsInt();
        int eventData = inputFields.get("event_data").getAsInt();
        Block block = Blocks.NOTE_BLOCK;
        String blockName = BuiltInRegistries.BLOCK.getKey(block).toString();
        if (!expectedBlock.equals(blockName)) {
            throw new IllegalArgumentException(
                "minimal block_event fixture expected " + expectedBlock
                    + " but official block is " + blockName
            );
        }
        BlockPos pos = new BlockPos(blockX, blockY, blockZ);
        ClientboundBlockEventPacket packet =
            new ClientboundBlockEventPacket(pos, block, eventType, eventData);
        RegistryAccess registryAccess =
            RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY);

        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundBlockEventPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundBlockEventPacket streamDecoded =
            ClientboundBlockEventPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] blockEventPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((packetType, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", packetType.id().toString());
            row.put("flow", packetType.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:block_event".equals(packetType.id().toString())) {
                blockEventPacketId[0] = packetId;
            }
        });
        if (blockEventPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound block_event packet id");
        }

        var protocolInfo = GameProtocols.CLIENTBOUND_TEMPLATE.bind(
            RegistryFriendlyByteBuf.decorator(registryAccess)
        );
        RegistryFriendlyByteBuf framedOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        protocolInfo.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);
        byte[] body = bytesAfterVarIntPrefix(framed);

        RegistryFriendlyByteBuf framedIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(framed), registryAccess);
        Packet<? super ClientGamePacketListener> decodedPacket =
            protocolInfo.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundBlockEventPacket decodedBlockEvent)) {
            throw new IllegalStateException(
                "decoded Play block_event as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        BlockPos streamDecodedPos = streamDecoded.getPos();
        BlockPos decodedPos = decodedBlockEvent.getPos();
        String streamDecodedBlock = BuiltInRegistries.BLOCK
            .getKey(streamDecoded.getBlock())
            .toString();
        String decodedBlock = BuiltInRegistries.BLOCK
            .getKey(decodedBlockEvent.getBlock())
            .toString();
        int blockRegistryId = BuiltInRegistries.BLOCK.getId(block);

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundBlockEventPacket(BlockPos, Block, int, int), ClientboundBlockEventPacket.STREAM_CODEC, private ClientboundBlockEventPacket(RegistryFriendlyByteBuf), private write(RegistryFriendlyByteBuf), RegistryFriendlyByteBuf.readBlockPos/writeBlockPos, RegistryFriendlyByteBuf.readUnsignedByte/writeByte, ByteBufCodecs.registry(Registries.BLOCK), BuiltInRegistries.BLOCK, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY))).codec().encode/decode(...), ClientGamePacketListener.handleBlockEvent(ClientboundBlockEventPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p -verbose net.minecraft.network.protocol.game.ClientboundBlockEventPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.world.level.block.Block net.minecraft.world.level.block.Blocks net.minecraft.core.registries.BuiltInRegistries"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:block_event");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundBlockEventPacket BlockPos, built-in Blocks.NOTE_BLOCK, event type, and event data constructor fixture; requires bootstrapped built-in registries but no initialized Level, BlockEntity, or game state");
        answerBody.put("official_body_shape", "block position encoded with RegistryFriendlyByteBuf BlockPos, event type encoded as one unsigned byte, event data encoded as one unsigned byte, and block encoded with ByteBufCodecs.registry(Registries.BLOCK)");
        answerBody.put("input_block_x", blockX);
        answerBody.put("input_block_y", blockY);
        answerBody.put("input_block_z", blockZ);
        answerBody.put("stream_decoded_block_x", streamDecodedPos.getX());
        answerBody.put("stream_decoded_block_y", streamDecodedPos.getY());
        answerBody.put("stream_decoded_block_z", streamDecodedPos.getZ());
        answerBody.put("decoded_block_x", decodedPos.getX());
        answerBody.put("decoded_block_y", decodedPos.getY());
        answerBody.put("decoded_block_z", decodedPos.getZ());
        answerBody.put("input_event_type", eventType);
        answerBody.put("stream_decoded_event_type", streamDecoded.getB0());
        answerBody.put("decoded_event_type", decodedBlockEvent.getB0());
        answerBody.put("input_event_data", eventData);
        answerBody.put("stream_decoded_event_data", streamDecoded.getB1());
        answerBody.put("decoded_event_data", decodedBlockEvent.getB1());
        answerBody.put("input_block", blockName);
        answerBody.put("stream_decoded_block", streamDecodedBlock);
        answerBody.put("decoded_block", decodedBlock);
        answerBody.put("decoded_block_registry_id", blockRegistryId);
        answerBody.put("remaining_after_packet_stream_decode", packetIn.readableBytes());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("fixture_body_hex", HexFormat.of().formatHex(fixtureBody));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("play_clientbound_packet_table", playClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> playBlockUpdateClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int blockX = inputFields.get("block_x").getAsInt();
        int blockY = inputFields.get("block_y").getAsInt();
        int blockZ = inputFields.get("block_z").getAsInt();
        String expectedBlock = inputFields.get("block").getAsString();
        BlockState blockState = Blocks.STONE.defaultBlockState();
        String blockName = BuiltInRegistries.BLOCK.getKey(blockState.getBlock()).toString();
        if (!expectedBlock.equals(blockName)) {
            throw new IllegalArgumentException(
                "minimal block_update fixture expected " + expectedBlock
                    + " but official block is " + blockName
            );
        }
        BlockPos pos = new BlockPos(blockX, blockY, blockZ);
        ClientboundBlockUpdatePacket packet =
            new ClientboundBlockUpdatePacket(pos, blockState);
        RegistryAccess registryAccess = RegistryAccess.EMPTY;

        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundBlockUpdatePacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundBlockUpdatePacket streamDecoded =
            ClientboundBlockUpdatePacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] blockUpdatePacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((packetType, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", packetType.id().toString());
            row.put("flow", packetType.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:block_update".equals(packetType.id().toString())) {
                blockUpdatePacketId[0] = packetId;
            }
        });
        if (blockUpdatePacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound block_update packet id");
        }

        var protocolInfo = GameProtocols.CLIENTBOUND_TEMPLATE.bind(
            RegistryFriendlyByteBuf.decorator(registryAccess)
        );
        RegistryFriendlyByteBuf framedOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        protocolInfo.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);
        byte[] body = bytesAfterVarIntPrefix(framed);

        RegistryFriendlyByteBuf framedIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(framed), registryAccess);
        Packet<? super ClientGamePacketListener> decodedPacket =
            protocolInfo.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundBlockUpdatePacket decodedBlockUpdate)) {
            throw new IllegalStateException(
                "decoded Play block_update as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        BlockPos streamDecodedPos = streamDecoded.getPos();
        BlockPos decodedPos = decodedBlockUpdate.getPos();
        String streamDecodedBlock = BuiltInRegistries.BLOCK
            .getKey(streamDecoded.getBlockState().getBlock())
            .toString();
        String decodedBlock = BuiltInRegistries.BLOCK
            .getKey(decodedBlockUpdate.getBlockState().getBlock())
            .toString();
        int blockStateRegistryId = Block.getId(blockState);

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundBlockUpdatePacket(BlockPos, BlockState), ClientboundBlockUpdatePacket.STREAM_CODEC, BlockPos.STREAM_CODEC, ByteBufCodecs.idMapper(Block.BLOCK_STATE_REGISTRY), Blocks.STONE.defaultBlockState(), Block.getId(BlockState), GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...), ClientGamePacketListener.handleBlockUpdate(ClientboundBlockUpdatePacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.world.level.block.state.BlockState net.minecraft.world.level.block.Blocks net.minecraft.world.level.block.Block"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:block_update");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundBlockUpdatePacket BlockPos and built-in Blocks.STONE.defaultBlockState() constructor fixture; requires bootstrapped built-in block state registry but no initialized Level or game state");
        answerBody.put("official_body_shape", "block position encoded with BlockPos.STREAM_CODEC and block state encoded with ByteBufCodecs.idMapper(Block.BLOCK_STATE_REGISTRY)");
        answerBody.put("input_block_x", blockX);
        answerBody.put("input_block_y", blockY);
        answerBody.put("input_block_z", blockZ);
        answerBody.put("stream_decoded_block_x", streamDecodedPos.getX());
        answerBody.put("stream_decoded_block_y", streamDecodedPos.getY());
        answerBody.put("stream_decoded_block_z", streamDecodedPos.getZ());
        answerBody.put("decoded_block_x", decodedPos.getX());
        answerBody.put("decoded_block_y", decodedPos.getY());
        answerBody.put("decoded_block_z", decodedPos.getZ());
        answerBody.put("input_block", blockName);
        answerBody.put("stream_decoded_block_state_block", streamDecodedBlock);
        answerBody.put("decoded_block_state_block", decodedBlock);
        answerBody.put("decoded_block_state_registry_id", blockStateRegistryId);
        answerBody.put("remaining_after_packet_stream_decode", packetIn.readableBytes());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("fixture_body_hex", HexFormat.of().formatHex(fixtureBody));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("play_clientbound_packet_table", playClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> playBossEventClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        UUID bossEventId = UUID.fromString(inputFields.get("uuid").getAsString());
        String operation = inputFields.get("operation").getAsString();
        if (!"REMOVE".equals(operation)) {
            throw new IllegalArgumentException("minimal boss_event fixture is scoped to the REMOVE operation");
        }

        ClientboundBossEventPacket packet = ClientboundBossEventPacket.createRemovePacket(bossEventId);
        RegistryAccess registryAccess = RegistryAccess.EMPTY;

        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundBossEventPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundBossEventPacket streamDecoded =
            ClientboundBossEventPacket.STREAM_CODEC.decode(packetIn);
        BossEventDispatchCapture streamCapture = BossEventDispatchCapture.capture(streamDecoded);

        RegistryFriendlyByteBuf operationReader =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        UUID bodyUuid = operationReader.readUUID();
        int operationOrdinal = operationReader.readVarInt();

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] bossEventPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((packetType, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", packetType.id().toString());
            row.put("flow", packetType.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:boss_event".equals(packetType.id().toString())) {
                bossEventPacketId[0] = packetId;
            }
        });
        if (bossEventPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound boss_event packet id");
        }

        var protocolInfo = GameProtocols.CLIENTBOUND_TEMPLATE.bind(
            RegistryFriendlyByteBuf.decorator(registryAccess)
        );
        RegistryFriendlyByteBuf framedOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        protocolInfo.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);
        byte[] body = bytesAfterVarIntPrefix(framed);

        RegistryFriendlyByteBuf framedIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(framed), registryAccess);
        Packet<? super ClientGamePacketListener> decodedPacket =
            protocolInfo.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundBossEventPacket decodedBossEvent)) {
            throw new IllegalStateException(
                "decoded Play boss_event as unexpected packet " + decodedPacket.getClass().getName()
            );
        }
        BossEventDispatchCapture decodedCapture = BossEventDispatchCapture.capture(decodedBossEvent);

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundBossEventPacket.createRemovePacket(UUID), ClientboundBossEventPacket.STREAM_CODEC, private write(RegistryFriendlyByteBuf), private ClientboundBossEventPacket(RegistryFriendlyByteBuf), RegistryFriendlyByteBuf.writeUUID/readUUID, RegistryFriendlyByteBuf.writeEnum/readEnum, REMOVE_OPERATION.write(...), ClientboundBossEventPacket.dispatch(Handler), GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...), ClientGamePacketListener.handleBossUpdate(ClientboundBossEventPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundBossEventPacket 'net.minecraft.network.protocol.game.ClientboundBossEventPacket$OperationType' 'net.minecraft.network.protocol.game.ClientboundBossEventPacket$1' net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:boss_event");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundBossEventPacket.createRemovePacket(UUID) fixture; context-free remove operation with no initialized BossEvent, Level, or game state");
        answerBody.put("official_body_shape", "UUID encoded with RegistryFriendlyByteBuf.writeUUID, operation encoded with RegistryFriendlyByteBuf.writeEnum/readEnum, and REMOVE operation writes no additional body");
        answerBody.put("input_uuid", bossEventId.toString());
        answerBody.put("body_uuid", bodyUuid.toString());
        answerBody.put("stream_decoded_uuid", streamCapture.uuid.toString());
        answerBody.put("decoded_uuid", decodedCapture.uuid.toString());
        answerBody.put("input_operation", operation);
        answerBody.put("stream_decoded_operation", streamCapture.operation);
        answerBody.put("decoded_operation", decodedCapture.operation);
        answerBody.put("decoded_operation_ordinal", operationOrdinal);
        answerBody.put("remaining_after_operation_read", operationReader.readableBytes());
        answerBody.put("remaining_after_packet_stream_decode", packetIn.readableBytes());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("fixture_body_hex", HexFormat.of().formatHex(fixtureBody));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("play_clientbound_packet_table", playClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static final class BossEventDispatchCapture {
        private final String operation;
        private final UUID uuid;

        private BossEventDispatchCapture(String operation, UUID uuid) {
            this.operation = operation;
            this.uuid = uuid;
        }

        private static BossEventDispatchCapture capture(ClientboundBossEventPacket packet) {
            final BossEventDispatchCapture[] capture = { null };
            packet.dispatch(new ClientboundBossEventPacket.Handler() {
                @Override
                public void remove(UUID id) {
                    capture[0] = new BossEventDispatchCapture("REMOVE", id);
                }
            });
            if (capture[0] == null) {
                throw new IllegalStateException("expected boss_event REMOVE dispatch");
            }
            return capture[0];
        }
    }

    private static Map<String, Object> playChangeDifficultyClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String difficultyName = inputFields.get("difficulty").getAsString();
        boolean locked = inputFields.get("locked").getAsBoolean();
        Difficulty difficulty = Difficulty.valueOf(difficultyName);

        ClientboundChangeDifficultyPacket packet =
            new ClientboundChangeDifficultyPacket(difficulty, locked);
        RegistryAccess registryAccess = RegistryAccess.EMPTY;

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundChangeDifficultyPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundChangeDifficultyPacket streamDecoded =
            ClientboundChangeDifficultyPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] changeDifficultyPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((packetType, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", packetType.id().toString());
            row.put("flow", packetType.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:change_difficulty".equals(packetType.id().toString())) {
                changeDifficultyPacketId[0] = packetId;
            }
        });
        if (changeDifficultyPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound change_difficulty packet id");
        }

        var protocolInfo = GameProtocols.CLIENTBOUND_TEMPLATE.bind(
            RegistryFriendlyByteBuf.decorator(registryAccess)
        );
        RegistryFriendlyByteBuf framedOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        protocolInfo.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);
        byte[] body = bytesAfterVarIntPrefix(framed);

        RegistryFriendlyByteBuf framedIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(framed), registryAccess);
        Packet<? super ClientGamePacketListener> decodedPacket =
            protocolInfo.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundChangeDifficultyPacket decodedChangeDifficulty)) {
            throw new IllegalStateException(
                "decoded Play change_difficulty as unexpected packet "
                    + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundChangeDifficultyPacket(Difficulty, boolean), ClientboundChangeDifficultyPacket.STREAM_CODEC, Difficulty.STREAM_CODEC, ByteBufCodecs.BOOL, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...), ClientGamePacketListener.handleChangeDifficulty(ClientboundChangeDifficultyPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundChangeDifficultyPacket net.minecraft.world.Difficulty net.minecraft.network.codec.ByteBufCodecs net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:change_difficulty");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundChangeDifficultyPacket(Difficulty, boolean) constructor fixture; context-free difficulty/locked record with no initialized Level or game state");
        answerBody.put("official_body_shape", "difficulty encoded with Difficulty.STREAM_CODEC followed by locked encoded with ByteBufCodecs.BOOL");
        answerBody.put("input_difficulty", difficulty.name());
        answerBody.put("input_difficulty_serialized_name", difficulty.getSerializedName());
        answerBody.put("input_difficulty_id", difficulty.getId());
        answerBody.put("stream_decoded_difficulty", streamDecoded.difficulty().name());
        answerBody.put("stream_decoded_difficulty_serialized_name", streamDecoded.difficulty().getSerializedName());
        answerBody.put("stream_decoded_difficulty_id", streamDecoded.difficulty().getId());
        answerBody.put("decoded_difficulty", decodedChangeDifficulty.difficulty().name());
        answerBody.put("decoded_difficulty_serialized_name", decodedChangeDifficulty.difficulty().getSerializedName());
        answerBody.put("decoded_difficulty_id", decodedChangeDifficulty.difficulty().getId());
        answerBody.put("input_locked", locked);
        answerBody.put("stream_decoded_locked", streamDecoded.locked());
        answerBody.put("decoded_locked", decodedChangeDifficulty.locked());
        answerBody.put("remaining_after_packet_stream_decode", packetIn.readableBytes());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("fixture_body_hex", HexFormat.of().formatHex(fixtureBody));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("play_clientbound_packet_table", playClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationSelectKnownPacksFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String vanillaPackId = inputFields.get("vanilla_pack_id").getAsString();
        List<KnownPack> knownPacks = List.of(KnownPack.vanilla(vanillaPackId));
        ServerboundSelectKnownPacks packet = new ServerboundSelectKnownPacks(knownPacks);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.SERVERBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ServerConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.SERVERBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ServerboundSelectKnownPacks decodedSelectKnownPacks)) {
            throw new IllegalStateException(
                "expected ServerboundSelectKnownPacks, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundSelectKnownPacks.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationServerboundPackets = new ArrayList<>();
        ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationServerboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "KnownPack.vanilla(String), ServerboundSelectKnownPacks(List<KnownPack>), ServerboundSelectKnownPacks.STREAM_CODEC, ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundSelectKnownPacks), ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), ServerboundSelectKnownPacks.knownPacks(), KnownPack.namespace(), KnownPack.id(), KnownPack.version(), KnownPack.isVanilla()",
            "decompiled_source_path", "_analysis/minecraft-26.1.2/decompiled-protocol/net/minecraft/network/protocol/configuration/ServerboundSelectKnownPacks.java"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Serverbound");
        answerBody.put("packet_type", "minecraft:select_known_packs");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_known_packs", knownPackAnswers(knownPacks));
        answerBody.put("decoded_known_packs", knownPackAnswers(decodedSelectKnownPacks.knownPacks()));
        answerBody.put("input_known_pack_count", knownPacks.size());
        answerBody.put("decoded_known_pack_count", decodedSelectKnownPacks.knownPacks().size());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_serverbound_packet_table", configurationServerboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationCustomClickActionFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        Identifier id = Identifier.parse(inputFields.get("id").getAsString());
        CompoundTag payload = new CompoundTag();
        payload.putString("source", inputFields.get("payload_source").getAsString());
        payload.putString("action", inputFields.get("payload_action").getAsString());
        Optional<Tag> optionalPayload = Optional.of(payload);
        ServerboundCustomClickActionPacket packet =
            new ServerboundCustomClickActionPacket(id, optionalPayload);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.SERVERBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ServerConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.SERVERBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ServerboundCustomClickActionPacket decodedCustomClickAction)) {
            throw new IllegalStateException(
                "expected ServerboundCustomClickActionPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundCustomClickActionPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationServerboundPackets = new ArrayList<>();
        ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationServerboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "Identifier.parse(String), CompoundTag.putString(String, String), ServerboundCustomClickActionPacket(Identifier, Optional<Tag>), ServerboundCustomClickActionPacket.STREAM_CODEC, ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundCustomClickActionPacket), ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), ServerboundCustomClickActionPacket.id(), ServerboundCustomClickActionPacket.payload(), Tag.getId(), Tag.getType(), Tag.toString()",
            "decompiled_source_path", "_analysis/minecraft-26.1.2/decompiled-protocol/net/minecraft/network/protocol/common/ServerboundCustomClickActionPacket.java"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Serverbound");
        answerBody.put("packet_type", "minecraft:custom_click_action");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_custom_click_id", id.toString());
        answerBody.put("decoded_custom_click_id", decodedCustomClickAction.id().toString());
        answerBody.put("input_payload_present", optionalPayload.isPresent());
        answerBody.put("decoded_payload_present", decodedCustomClickAction.payload().isPresent());
        answerBody.put("input_payload_tag_id", payload.getId());
        answerBody.put(
            "decoded_payload_tag_id",
            decodedCustomClickAction.payload().map(tag -> (int) tag.getId()).orElse(-1)
        );
        answerBody.put("input_payload_type", payload.getType().getName());
        answerBody.put(
            "decoded_payload_type",
            decodedCustomClickAction.payload().map(tag -> tag.getType().getName()).orElse("")
        );
        answerBody.put("input_payload_snbt", payload.toString());
        answerBody.put(
            "decoded_payload_snbt",
            decodedCustomClickAction.payload().map(Tag::toString).orElse("")
        );
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_serverbound_packet_table", configurationServerboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> configurationAcceptCodeOfConductFramedDispatch(JsonObject input) {
        ServerboundAcceptCodeOfConductPacket packet = ServerboundAcceptCodeOfConductPacket.INSTANCE;

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.SERVERBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ServerConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.SERVERBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ServerboundAcceptCodeOfConductPacket decodedAcceptCodeOfConduct)) {
            throw new IllegalStateException(
                "expected ServerboundAcceptCodeOfConductPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundAcceptCodeOfConductPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> configurationServerboundPackets = new ArrayList<>();
        ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationServerboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ServerboundAcceptCodeOfConductPacket.INSTANCE, ServerboundAcceptCodeOfConductPacket.STREAM_CODEC, ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundAcceptCodeOfConductPacket), ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), ServerboundAcceptCodeOfConductPacket.type()",
            "decompiled_source_path", "_analysis/minecraft-26.1.2/decompiled-protocol/net/minecraft/network/protocol/configuration/ServerboundAcceptCodeOfConductPacket.java"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Serverbound");
        answerBody.put("packet_type", "minecraft:accept_code_of_conduct");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("instance_packet_type", packet.type().id().toString());
        answerBody.put("decoded_equals_instance", decodedAcceptCodeOfConduct.equals(packet));
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_serverbound_packet_table", configurationServerboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> loginHelloClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String serverId = inputFields.get("server_id").getAsString();
        byte[] publicKey = hexToBytes(inputFields.get("public_key_hex").getAsString());
        byte[] challenge = hexToBytes(inputFields.get("challenge_hex").getAsString());
        boolean shouldAuthenticate = inputFields.get("should_authenticate").getAsBoolean();
        ClientboundHelloPacket packet = new ClientboundHelloPacket(
            serverId,
            publicKey,
            challenge,
            shouldAuthenticate
        );

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        LoginProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientLoginPacketListener> decodedPacket =
            LoginProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundHelloPacket decodedHello)) {
            throw new IllegalStateException(
                "expected ClientboundHelloPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundHelloPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        byte[] decodedPublicKey = privateByteArray(decodedHello, "publicKey");

        List<Map<String, Object>> loginClientboundPackets = new ArrayList<>();
        LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            loginClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundHelloPacket(String, byte[], byte[], boolean), ClientboundHelloPacket.STREAM_CODEC, LoginProtocols.CLIENTBOUND.codec().encode/decode(ClientboundHelloPacket), LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundHelloPacket.getServerId(), getChallenge(), shouldAuthenticate(), private publicKey field, ClientLoginPacketListener.handleHello(ClientboundHelloPacket)",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ClientboundHelloPacket net.minecraft.network.protocol.login.ClientLoginPacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Login");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:hello");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_server_id", serverId);
        answerBody.put("decoded_server_id", decodedHello.getServerId());
        answerBody.put("input_public_key_hex", HexFormat.of().formatHex(publicKey));
        answerBody.put("decoded_public_key_hex", HexFormat.of().formatHex(decodedPublicKey));
        answerBody.put("input_public_key_length", publicKey.length);
        answerBody.put("decoded_public_key_length", decodedPublicKey.length);
        answerBody.put("input_challenge_hex", HexFormat.of().formatHex(challenge));
        answerBody.put("decoded_challenge_hex", HexFormat.of().formatHex(decodedHello.getChallenge()));
        answerBody.put("input_challenge_length", challenge.length);
        answerBody.put("decoded_challenge_length", decodedHello.getChallenge().length);
        answerBody.put("input_should_authenticate", shouldAuthenticate);
        answerBody.put("decoded_should_authenticate", decodedHello.shouldAuthenticate());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("login_clientbound_packet_table", loginClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> loginFinishedClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        UUID profileId = UUID.fromString(inputFields.get("profile_id").getAsString());
        String profileName = inputFields.get("profile_name").getAsString();
        GameProfile gameProfile = new GameProfile(profileId, profileName);
        ClientboundLoginFinishedPacket packet = new ClientboundLoginFinishedPacket(gameProfile);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        LoginProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientLoginPacketListener> decodedPacket =
            LoginProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundLoginFinishedPacket decodedLoginFinished)) {
            throw new IllegalStateException(
                "expected ClientboundLoginFinishedPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundLoginFinishedPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> loginClientboundPackets = new ArrayList<>();
        LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            loginClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "GameProfile(UUID, String), ClientboundLoginFinishedPacket(GameProfile), ClientboundLoginFinishedPacket.STREAM_CODEC, ByteBufCodecs.GAME_PROFILE, LoginProtocols.CLIENTBOUND.codec().encode/decode(ClientboundLoginFinishedPacket), LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundLoginFinishedPacket.gameProfile(), isTerminal(), ClientLoginPacketListener.handleLoginFinished(ClientboundLoginFinishedPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket net.minecraft.network.protocol.login.ClientLoginPacketListener net.minecraft.network.codec.ByteBufCodecs 'net.minecraft.network.codec.ByteBufCodecs$32' com.mojang.authlib.GameProfile com.mojang.authlib.properties.PropertyMap"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Login");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:login_finished");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_profile_id", profileId.toString());
        answerBody.put("decoded_profile_id", decodedLoginFinished.gameProfile().id().toString());
        answerBody.put("input_profile_name", profileName);
        answerBody.put("decoded_profile_name", decodedLoginFinished.gameProfile().name());
        answerBody.put("input_property_count", gameProfile.properties().size());
        answerBody.put("decoded_property_count", decodedLoginFinished.gameProfile().properties().size());
        answerBody.put("input_is_terminal", packet.isTerminal());
        answerBody.put("decoded_is_terminal", decodedLoginFinished.isTerminal());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("login_clientbound_packet_table", loginClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> loginCompressionClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int compressionThreshold = inputFields.get("compression_threshold").getAsInt();
        ClientboundLoginCompressionPacket packet =
            new ClientboundLoginCompressionPacket(compressionThreshold);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        LoginProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientLoginPacketListener> decodedPacket =
            LoginProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundLoginCompressionPacket decodedCompression)) {
            throw new IllegalStateException(
                "expected ClientboundLoginCompressionPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundLoginCompressionPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        List<Map<String, Object>> loginClientboundPackets = new ArrayList<>();
        LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            loginClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundLoginCompressionPacket(int), ClientboundLoginCompressionPacket.STREAM_CODEC, FriendlyByteBuf.readVarInt/writeVarInt, LoginProtocols.CLIENTBOUND.codec().encode/decode(ClientboundLoginCompressionPacket), LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundLoginCompressionPacket.getCompressionThreshold(), ClientLoginPacketListener.handleCompression(ClientboundLoginCompressionPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ClientboundLoginCompressionPacket net.minecraft.network.protocol.login.ClientLoginPacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Login");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:login_compression");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_compression_threshold", compressionThreshold);
        answerBody.put("decoded_compression_threshold", decodedCompression.getCompressionThreshold());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("login_clientbound_packet_table", loginClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static Map<String, Object> loginCustomQueryClientboundFramedDispatch(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int transactionId = inputFields.get("transaction_id").getAsInt();
        String payloadIdText = inputFields.get("payload_id").getAsString();
        Identifier payloadId = Identifier.parse(payloadIdText);
        DiscardedQueryPayload payload = new DiscardedQueryPayload(payloadId);
        ClientboundCustomQueryPacket packet =
            new ClientboundCustomQueryPacket(transactionId, payload);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        LoginProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientLoginPacketListener> decodedPacket =
            LoginProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundCustomQueryPacket decodedCustomQuery)) {
            throw new IllegalStateException(
                "expected ClientboundCustomQueryPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundCustomQueryPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);
        byte[] payloadBody = bytesAfterVarIntAndIdentifierPrefix(body);

        List<Map<String, Object>> loginClientboundPackets = new ArrayList<>();
        LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            loginClientboundPackets.add(row);
        });

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "Identifier.parse(String), DiscardedQueryPayload(Identifier), ClientboundCustomQueryPacket(int, CustomQueryPayload), ClientboundCustomQueryPacket.STREAM_CODEC, FriendlyByteBuf.readVarInt/writeVarInt, FriendlyByteBuf.readIdentifier/writeIdentifier, LoginProtocols.CLIENTBOUND.codec().encode/decode(ClientboundCustomQueryPacket), LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundCustomQueryPacket.transactionId(), ClientboundCustomQueryPacket.payload(), DiscardedQueryPayload.id(), CustomQueryPayload.write(FriendlyByteBuf), ClientLoginPacketListener.handleCustomQuery(ClientboundCustomQueryPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.login.ClientboundCustomQueryPacket net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ClientLoginPacketListener net.minecraft.network.protocol.login.custom.CustomQueryPayload net.minecraft.network.protocol.login.custom.DiscardedQueryPayload"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Login");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:custom_query");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_transaction_id", transactionId);
        answerBody.put("decoded_transaction_id", decodedCustomQuery.transactionId());
        answerBody.put("input_payload_id", payload.id().toString());
        answerBody.put("decoded_payload_id", decodedCustomQuery.payload().id().toString());
        answerBody.put("input_payload_class", payload.getClass().getName());
        answerBody.put("decoded_payload_class", decodedCustomQuery.payload().getClass().getName());
        answerBody.put("input_payload_length", payloadBody.length);
        answerBody.put("decoded_payload_length", payloadBody.length);
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("encoded_payload_body_hex", HexFormat.of().formatHex(payloadBody));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("login_clientbound_packet_table", loginClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }

    private static byte[] readableBytes(FriendlyByteBuf buffer) {
        byte[] bytes = new byte[buffer.readableBytes()];
        buffer.getBytes(buffer.readerIndex(), bytes);
        return bytes;
    }

    private static byte[] bytesAfterVarIntPrefix(byte[] framed) {
        int offset = 0;
        for (; offset < framed.length && offset < 5; offset += 1) {
            if ((framed[offset] & 0x80) == 0) {
                offset += 1;
                break;
            }
        }
        if (offset == 0 || offset > framed.length || offset > 5) {
            throw new IllegalStateException("missing complete VarInt packet id prefix");
        }
        byte[] body = new byte[framed.length - offset];
        System.arraycopy(framed, offset, body, 0, body.length);
        return body;
    }

    private static byte[] bytesAfterVarIntAndIdentifierPrefix(byte[] body) {
        int offset = varIntPrefixLength(body, 0);
        int identifierLengthPrefix = varIntPrefixLength(body, offset);
        int identifierLength = readVarInt(body, offset);
        offset += identifierLengthPrefix + identifierLength;
        if (offset > body.length) {
            throw new IllegalStateException("identifier length extends past body");
        }
        byte[] payloadBody = new byte[body.length - offset];
        System.arraycopy(body, offset, payloadBody, 0, payloadBody.length);
        return payloadBody;
    }

    private static int varIntPrefixLength(byte[] bytes, int offset) {
        for (int i = offset; i < bytes.length && i < offset + 5; i += 1) {
            if ((bytes[i] & 0x80) == 0) {
                return i - offset + 1;
            }
        }
        throw new IllegalStateException("missing complete VarInt prefix");
    }

    private static int readVarInt(byte[] bytes, int offset) {
        int value = 0;
        int shift = 0;
        for (int i = offset; i < bytes.length && i < offset + 5; i += 1) {
            int current = bytes[i] & 0xFF;
            value |= (current & 0x7F) << shift;
            if ((current & 0x80) == 0) {
                return value;
            }
            shift += 7;
        }
        throw new IllegalStateException("missing complete VarInt value");
    }

    private static byte privateByte(Object target, String fieldName) {
        try {
            Field field = target.getClass().getDeclaredField(fieldName);
            field.setAccessible(true);
            return field.getByte(target);
        } catch (ReflectiveOperationException err) {
            throw new IllegalStateException(
                "failed to read private byte field " + fieldName + " from " + target.getClass().getName(),
                err
            );
        }
    }

    private static Map<String, Object> finishDirectionAnswer(
        String flow,
        String packetType,
        Packet<?> decodedPacket,
        boolean instanceTerminal,
        boolean decodedTerminal,
        byte[] framed,
        int remainingAfterDecode,
        List<Map<String, Object>> packetTable
    ) {
        Map<String, Object> row = new LinkedHashMap<>();
        row.put("flow", flow);
        row.put("packet_type", packetType);
        row.put("decoded_packet_type", decodedPacket.type().id().toString());
        row.put("decoded_packet_class", decodedPacket.getClass().getName());
        row.put("instance_is_terminal", instanceTerminal);
        row.put("decoded_is_terminal", decodedTerminal);
        row.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        row.put("encoded_body_hex", HexFormat.of().formatHex(bytesAfterVarIntPrefix(framed)));
        row.put("remaining_after_official_decode", remainingAfterDecode);
        row.put("configuration_packet_table", packetTable);
        return row;
    }

    private static Map<String, Object> framedDirectionAnswer(
        String flow,
        String packetType,
        Packet<?> decodedPacket,
        int inputId,
        int decodedId,
        byte[] framed,
        byte[] body,
        int remainingAfterDecode,
        List<Map<String, Object>> packetTable
    ) {
        Map<String, Object> row = new LinkedHashMap<>();
        row.put("flow", flow);
        row.put("packet_type", packetType);
        row.put("decoded_packet_type", decodedPacket.type().id().toString());
        row.put("decoded_packet_class", decodedPacket.getClass().getName());
        row.put("input_id", inputId);
        row.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        row.put("encoded_body_hex", HexFormat.of().formatHex(body));
        row.put("decoded_id", decodedId);
        row.put("remaining_after_official_decode", remainingAfterDecode);
        row.put("configuration_packet_table", packetTable);
        return row;
    }

    private static Map<String, Object> clientInformationAnswer(ClientInformation information) {
        Map<String, Object> row = new LinkedHashMap<>();
        row.put("language", information.language());
        row.put("view_distance", information.viewDistance());
        row.put("chat_visibility", information.chatVisibility().name());
        row.put("chat_colors", information.chatColors());
        row.put("model_customisation", information.modelCustomisation());
        row.put("main_hand", information.mainHand().name());
        row.put("text_filtering_enabled", information.textFilteringEnabled());
        row.put("allows_listing", information.allowsListing());
        row.put("particle_status", information.particleStatus().name());
        return row;
    }

    private static List<Map<String, Object>> knownPackAnswers(List<KnownPack> knownPacks) {
        List<Map<String, Object>> rows = new ArrayList<>();
        for (KnownPack knownPack : knownPacks) {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("namespace", knownPack.namespace());
            row.put("id", knownPack.id());
            row.put("version", knownPack.version());
            row.put("is_vanilla", knownPack.isVanilla());
            rows.add(row);
        }
        return rows;
    }

    private static List<Map<String, Object>> serverLinkAnswers(List<ServerLinks.UntrustedEntry> links) {
        List<Map<String, Object>> rows = new ArrayList<>();
        for (ServerLinks.UntrustedEntry link : links) {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("link", link.link());
            row.put("type", link.type().map(knownType -> {
                Map<String, Object> known = new LinkedHashMap<>();
                known.put("kind", "known");
                known.put("name", knownType.name());
                return known;
            }, customName -> {
                Map<String, Object> custom = new LinkedHashMap<>();
                custom.put("kind", "custom");
                custom.put("text", customName.getString());
                return custom;
            }));
            rows.add(row);
        }
        return rows;
    }

    private static List<String> identifierStrings(Set<Identifier> identifiers) {
        return identifiers.stream().map(Identifier::toString).sorted().toList();
    }

    private static byte[] hexToBytes(String value) {
        if (value.isEmpty()) {
            return new byte[0];
        }
        return HexFormat.of().parseHex(value);
    }

    private static byte[] privateByteArray(Object object, String fieldName) {
        try {
            var field = object.getClass().getDeclaredField(fieldName);
            field.setAccessible(true);
            return (byte[]) field.get(object);
        } catch (ReflectiveOperationException err) {
            throw new IllegalStateException(
                "failed to read official field " + fieldName + " from " + object.getClass().getName(),
                err
            );
        }
    }

    private static ClientboundBlockEntityDataPacket constructBlockEntityDataPacket(
        BlockPos pos,
        BlockEntityType<?> type,
        CompoundTag tag
    ) {
        try {
            Constructor<ClientboundBlockEntityDataPacket> constructor =
                ClientboundBlockEntityDataPacket.class.getDeclaredConstructor(
                    BlockPos.class,
                    BlockEntityType.class,
                    CompoundTag.class
                );
            constructor.setAccessible(true);
            return constructor.newInstance(pos, type, tag);
        } catch (ReflectiveOperationException err) {
            throw new IllegalStateException("failed to call official ClientboundBlockEntityDataPacket constructor", err);
        }
    }
}
