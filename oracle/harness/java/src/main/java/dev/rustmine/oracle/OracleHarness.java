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
import net.minecraft.network.protocol.configuration.ConfigurationProtocols;
import net.minecraft.network.protocol.configuration.ServerConfigurationPacketListener;
import net.minecraft.network.protocol.common.ServerboundKeepAlivePacket;
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
}
