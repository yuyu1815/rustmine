package dev.rustmine.oracle.cases.configuration;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.time.Instant;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.configuration.ClientConfigurationPacketListener;
import net.minecraft.network.protocol.configuration.ConfigurationProtocols;
import net.minecraft.network.protocol.configuration.ServerConfigurationPacketListener;
import net.minecraft.network.protocol.common.ClientboundPingPacket;
import net.minecraft.network.protocol.common.ServerboundPongPacket;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class ConfigurationPingPongFramedDispatchCase {
    private ConfigurationPingPongFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
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
}
