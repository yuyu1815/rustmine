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
import net.minecraft.network.protocol.login.ClientboundLoginDisconnectPacket;
import net.minecraft.network.chat.Component;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class LoginDisconnectClientboundFramedDispatchCase {
    private LoginDisconnectClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String reasonText = inputFields.get("reason_text").getAsString();
        Component reason = Component.literal(reasonText);
        ClientboundLoginDisconnectPacket packet = new ClientboundLoginDisconnectPacket(reason);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        LoginProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientLoginPacketListener> decodedPacket =
            LoginProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundLoginDisconnectPacket decodedDisconnect)) {
            throw new IllegalStateException(
                "expected ClientboundLoginDisconnectPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundLoginDisconnectPacket.STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);

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
            "function_or_member", "Component.literal(String), ClientboundLoginDisconnectPacket(Component), ClientboundLoginDisconnectPacket.STREAM_CODEC, LoginProtocols.CLIENTBOUND.codec().encode/decode(ClientboundLoginDisconnectPacket), LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundLoginDisconnectPacket.reason(), Component.getString(), ClientLoginPacketListener extends ClientCookiePacketListener",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ClientboundLoginDisconnectPacket net.minecraft.network.protocol.login.ClientLoginPacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Login");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:login_disconnect");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("reason_fixture", "Component.literal(\"\")");
        answerBody.put("input_reason_text", reason.getString());
        answerBody.put("decoded_reason_text", decodedDisconnect.reason().getString());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("login_clientbound_packet_table", loginClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }
}
