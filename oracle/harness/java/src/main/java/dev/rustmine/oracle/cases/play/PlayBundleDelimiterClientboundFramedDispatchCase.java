package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.time.Instant;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlayBundleDelimiterClientboundFramedDispatchCase {
    private PlayBundleDelimiterClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
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
}
