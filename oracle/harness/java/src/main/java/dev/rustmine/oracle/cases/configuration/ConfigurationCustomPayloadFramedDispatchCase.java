package dev.rustmine.oracle.cases.configuration;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.time.Instant;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.configuration.ConfigurationProtocols;
import net.minecraft.network.protocol.configuration.ServerConfigurationPacketListener;
import net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket;
import net.minecraft.network.protocol.common.custom.BrandPayload;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class ConfigurationCustomPayloadFramedDispatchCase {
    private ConfigurationCustomPayloadFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
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
}
