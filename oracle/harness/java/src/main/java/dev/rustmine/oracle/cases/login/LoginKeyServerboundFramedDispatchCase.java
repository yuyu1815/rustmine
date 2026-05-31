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
import net.minecraft.network.protocol.login.ServerLoginPacketListener;
import net.minecraft.network.protocol.login.ServerboundKeyPacket;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class LoginKeyServerboundFramedDispatchCase {
    private LoginKeyServerboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        byte[] keybytes = hexToBytes(inputFields.get("keybytes_hex").getAsString());
        byte[] encryptedChallenge = hexToBytes(inputFields.get("encrypted_challenge_hex").getAsString());

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        fixtureBodyOut.writeByteArray(keybytes);
        fixtureBodyOut.writeByteArray(encryptedChallenge);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        ServerboundKeyPacket packet = ServerboundKeyPacket.STREAM_CODEC.decode(
            new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody))
        );

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        LoginProtocols.SERVERBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ServerLoginPacketListener> decodedPacket =
            LoginProtocols.SERVERBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ServerboundKeyPacket decodedKey)) {
            throw new IllegalStateException(
                "expected ServerboundKeyPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ServerboundKeyPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        byte[] decodedKeybytes = privateByteArray(decodedKey, "keybytes");
        byte[] decodedEncryptedChallenge = privateByteArray(decodedKey, "encryptedChallenge");

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
            "function_or_member", "ServerboundKeyPacket.STREAM_CODEC, LoginProtocols.SERVERBOUND.codec().encode/decode(ServerboundKeyPacket), LoginProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...), private ServerboundKeyPacket(FriendlyByteBuf), private write(FriendlyByteBuf), keybytes, encryptedChallenge",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ServerboundKeyPacket"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Login");
        answerBody.put("flow", "Serverbound");
        answerBody.put("packet_type", "minecraft:key");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_keybytes_hex", HexFormat.of().formatHex(keybytes));
        answerBody.put("decoded_keybytes_hex", HexFormat.of().formatHex(decodedKeybytes));
        answerBody.put("input_keybytes_length", keybytes.length);
        answerBody.put("decoded_keybytes_length", decodedKeybytes.length);
        answerBody.put("input_encrypted_challenge_hex", HexFormat.of().formatHex(encryptedChallenge));
        answerBody.put("decoded_encrypted_challenge_hex", HexFormat.of().formatHex(decodedEncryptedChallenge));
        answerBody.put("input_encrypted_challenge_length", encryptedChallenge.length);
        answerBody.put("decoded_encrypted_challenge_length", decodedEncryptedChallenge.length);
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("login_serverbound_packet_table", loginServerboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }
}
