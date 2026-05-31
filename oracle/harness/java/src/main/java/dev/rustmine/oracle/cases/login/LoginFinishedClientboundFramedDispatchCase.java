package dev.rustmine.oracle.cases.login;

import com.google.gson.JsonObject;
import com.mojang.authlib.GameProfile;
import io.netty.buffer.Unpooled;
import java.time.Instant;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.login.LoginProtocols;
import net.minecraft.network.protocol.login.ClientLoginPacketListener;
import net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class LoginFinishedClientboundFramedDispatchCase {
    private LoginFinishedClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        UUID profileId = UUID.fromString(inputFields.get("profile_id").getAsString());
        String profileName = inputFields.get("profile_name").getAsString();
        GameProfile gameProfile = new GameProfile(profileId, profileName);
        ClientboundLoginFinishedPacket packet = new ClientboundLoginFinishedPacket(gameProfile);

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        LoginProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientLoginPacketListener> decodedPacket =
            LoginProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundLoginFinishedPacket decodedLoginFinished)) {
            throw new IllegalStateException(
                "expected ClientboundLoginFinishedPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundLoginFinishedPacket.STREAM_CODEC.encode(bodyOut, packet);
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
            "function_or_member", "GameProfile(UUID, String), ClientboundLoginFinishedPacket(GameProfile), ClientboundLoginFinishedPacket.STREAM_CODEC, ByteBufCodecs.GAME_PROFILE, LoginProtocols.CLIENTBOUND.codec().encode/decode(ClientboundLoginFinishedPacket), LoginProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundLoginFinishedPacket.gameProfile(), isTerminal(), ClientLoginPacketListener.handleLoginFinished(ClientboundLoginFinishedPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.login.LoginProtocols net.minecraft.network.protocol.login.LoginPacketTypes net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket net.minecraft.network.protocol.login.ClientLoginPacketListener net.minecraft.network.codec.ByteBufCodecs 'net.minecraft.network.codec.ByteBufCodecs$32' com.mojang.authlib.GameProfile com.mojang.authlib.properties.PropertyMap"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Login");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:login_finished");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_profile_id", profileId.toString());
        answerBody.put("decoded_profile_id", decodedLoginFinished.gameProfile().id().toString());
        answerBody.put("input_profile_name", profileName);
        answerBody.put("decoded_profile_name", decodedLoginFinished.gameProfile().name());
        answerBody.put("input_property_count", gameProfile.properties().size());
        answerBody.put("decoded_property_count", decodedLoginFinished.gameProfile().properties().size());
        answerBody.put("input_is_terminal", packet.isTerminal());
        answerBody.put("decoded_is_terminal", decodedLoginFinished.isTerminal());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("login_clientbound_packet_table", loginClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }
}
