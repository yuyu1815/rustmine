package dev.rustmine.oracle;

import com.google.gson.JsonObject;
import java.time.Instant;
import java.util.HexFormat;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.network.protocol.Packet;


public final class OraclePlayAnswers {
    private OraclePlayAnswers() {
    }

    public static Map<String, Object> playAnswerHeader(
        JsonObject input,
        String functionOrMember,
        String bytecodeSourceCommand
    ) {
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
            "function_or_member", functionOrMember,
            "bytecode_source_command", bytecodeSourceCommand
        ));
        return answer;
    }

    public static Map<String, Object> playAnswerBody(
        String packetType,
        Packet<?> decodedPacket,
        String fixture,
        String officialBodyShape,
        int packetId,
        int remainingAfterPacketStreamDecode,
        byte[] framed,
        byte[] body,
        byte[] fixtureBody,
        int remainingAfterOfficialDecode,
        List<Map<String, Object>> playClientboundPackets
    ) {
        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", packetType);
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", fixture);
        answerBody.put("official_body_shape", officialBodyShape);
        answerBody.put("official_packet_id", packetId);
        answerBody.put("remaining_after_packet_stream_decode", remainingAfterPacketStreamDecode);
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("fixture_body_hex", HexFormat.of().formatHex(fixtureBody));
        answerBody.put("remaining_after_official_decode", remainingAfterOfficialDecode);
        answerBody.put("play_clientbound_packet_table", playClientboundPackets);
        return answerBody;
    }
}
