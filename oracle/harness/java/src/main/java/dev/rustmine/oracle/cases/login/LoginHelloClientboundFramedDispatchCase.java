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
import net.minecraft.network.protocol.login.ClientLoginPacketListener;
import net.minecraft.network.protocol.login.ClientboundHelloPacket;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class LoginHelloClientboundFramedDispatchCase {
    private LoginHelloClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String serverId = inputFields.get("server_id").getAsString();
        byte[] publicKey = hexToBytes(inputFields.get("public_key_hex").getAsString());
        byte[] challenge = hexToBytes(inputFields.get("challenge_hex").getAsString());
        boolean shouldAuthenticate = inputFields.get("should_authenticate").getAsBoolean();
        ClientboundHelloPacket packet = new ClientboundHelloPacket(
            serverId,
            publicKey,
            challenge,
            shouldAuthenticate
        );

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        LoginProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientLoginPacketListener> decodedPacket =
            LoginProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundHelloPacket decodedHello)) {
            throw new IllegalStateException(
                "expected ClientboundHelloPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundHelloPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

        byte[] decodedPublicKey = privateByteArray(decodedHello, "publicKey");

        List<Map<String, Object>> loginClientboundPackets = new ArrayList<>();
        LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            loginClientboundPackets.add(row);
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
            "function_or_member", "ClientboundHelloPacket(String, byte[], byte[], boolean), ClientboundHelloPacket.STREAM_CODEC, LoginProtocols.CLIENTBOUND.codec().encode/decode(ClientboundHelloPacket), LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundHelloPacket.getServerId(), getChallenge(), shouldAuthenticate(), private publicKey field, ClientLoginPacketListener.handleHello(ClientboundHelloPacket)",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ClientboundHelloPacket net.minecraft.network.protocol.login.ClientLoginPacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Login");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:hello");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_server_id", serverId);
        answerBody.put("decoded_server_id", decodedHello.getServerId());
        answerBody.put("input_public_key_hex", HexFormat.of().formatHex(publicKey));
        answerBody.put("decoded_public_key_hex", HexFormat.of().formatHex(decodedPublicKey));
        answerBody.put("input_public_key_length", publicKey.length);
        answerBody.put("decoded_public_key_length", decodedPublicKey.length);
        answerBody.put("input_challenge_hex", HexFormat.of().formatHex(challenge));
        answerBody.put("decoded_challenge_hex", HexFormat.of().formatHex(decodedHello.getChallenge()));
        answerBody.put("input_challenge_length", challenge.length);
        answerBody.put("decoded_challenge_length", decodedHello.getChallenge().length);
        answerBody.put("input_should_authenticate", shouldAuthenticate);
        answerBody.put("decoded_should_authenticate", decodedHello.shouldAuthenticate());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("login_clientbound_packet_table", loginClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }
}
