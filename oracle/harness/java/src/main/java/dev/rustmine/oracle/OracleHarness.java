package dev.rustmine.oracle;

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.io.IOException;
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
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.configuration.ClientConfigurationPacketListener;
import net.minecraft.network.protocol.configuration.ClientboundFinishConfigurationPacket;
import net.minecraft.network.protocol.configuration.ClientboundRegistryDataPacket;
import net.minecraft.network.protocol.configuration.ConfigurationProtocols;
import net.minecraft.network.protocol.configuration.ClientboundResetChatPacket;
import net.minecraft.network.protocol.configuration.ClientboundUpdateEnabledFeaturesPacket;
import net.minecraft.network.protocol.configuration.ServerboundAcceptCodeOfConductPacket;
import net.minecraft.network.protocol.configuration.ServerConfigurationPacketListener;
import net.minecraft.network.protocol.configuration.ServerboundFinishConfigurationPacket;
import net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.common.ClientboundKeepAlivePacket;
import net.minecraft.network.protocol.common.ClientboundCustomPayloadPacket;
import net.minecraft.network.protocol.common.ClientboundDisconnectPacket;
import net.minecraft.network.protocol.common.ClientboundPingPacket;
import net.minecraft.network.protocol.common.ClientboundResourcePackPopPacket;
import net.minecraft.network.protocol.common.ClientboundResourcePackPushPacket;
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
import net.minecraft.core.Registry;
import net.minecraft.core.registries.Registries;
import net.minecraft.resources.Identifier;
import net.minecraft.resources.ResourceKey;
import net.minecraft.server.Bootstrap;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.packs.repository.KnownPack;
import net.minecraft.tags.TagNetworkSerialization;

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

    private static List<String> identifierStrings(Set<Identifier> identifiers) {
        return identifiers.stream().map(Identifier::toString).sorted().toList();
    }
}
