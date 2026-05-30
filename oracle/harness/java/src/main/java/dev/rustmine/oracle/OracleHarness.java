package dev.rustmine.oracle;

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Instant;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.network.protocol.Packet;
import net.minecraft.SharedConstants;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.protocol.configuration.ClientConfigurationPacketListener;
import net.minecraft.network.protocol.configuration.ClientboundFinishConfigurationPacket;
import net.minecraft.network.protocol.configuration.ConfigurationProtocols;
import net.minecraft.network.protocol.configuration.ServerConfigurationPacketListener;
import net.minecraft.network.protocol.configuration.ServerboundFinishConfigurationPacket;
import net.minecraft.network.protocol.common.ClientboundKeepAlivePacket;
import net.minecraft.network.protocol.common.ClientboundPingPacket;
import net.minecraft.network.protocol.common.ServerboundClientInformationPacket;
import net.minecraft.network.protocol.common.ServerboundKeepAlivePacket;
import net.minecraft.network.protocol.common.ServerboundPongPacket;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.Bootstrap;

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
}
