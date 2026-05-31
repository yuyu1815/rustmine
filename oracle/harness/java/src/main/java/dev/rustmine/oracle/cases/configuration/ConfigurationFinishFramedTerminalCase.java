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
import net.minecraft.network.protocol.configuration.ClientboundFinishConfigurationPacket;
import net.minecraft.network.protocol.configuration.ConfigurationProtocols;
import net.minecraft.network.protocol.configuration.ServerConfigurationPacketListener;
import net.minecraft.network.protocol.configuration.ServerboundFinishConfigurationPacket;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class ConfigurationFinishFramedTerminalCase {
    private ConfigurationFinishFramedTerminalCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
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
}
