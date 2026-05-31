package dev.rustmine.oracle.cases.handshake;

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
import net.minecraft.network.protocol.handshake.ClientIntent;
import net.minecraft.network.protocol.handshake.ClientIntentionPacket;
import net.minecraft.network.protocol.handshake.HandshakeProtocols;
import net.minecraft.network.protocol.handshake.ServerHandshakePacketListener;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class HandshakeIntentionFramedDispatchCase {
    private HandshakeIntentionFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
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
}
