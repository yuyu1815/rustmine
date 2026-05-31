package dev.rustmine.oracle.cases.login;

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
import net.minecraft.network.protocol.login.LoginProtocols;
import net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket;
import net.minecraft.network.protocol.login.ServerLoginPacketListener;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class LoginCustomQueryAnswerServerboundFramedDispatchCase {
    private LoginCustomQueryAnswerServerboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
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
}
