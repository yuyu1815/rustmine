package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.Arrays;
import java.util.HexFormat;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.common.ClientboundStoreCookiePacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.resources.Identifier;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;


public final class PlayStoreCookieClientboundFramedDispatchCase {
    private PlayStoreCookieClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        Identifier key = Identifier.parse(inputFields.get("key").getAsString());
        byte[] payload = HexFormat.of().parseHex(inputFields.get("payload_hex").getAsString());
        ClientboundStoreCookiePacket packet = new ClientboundStoreCookiePacket(key, payload);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundStoreCookiePacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundStoreCookiePacket streamDecoded =
            ClientboundStoreCookiePacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:store_cookie");

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
        var protocolInfo = GameProtocols.CLIENTBOUND_TEMPLATE.bind(
            RegistryFriendlyByteBuf.decorator(registryAccess)
        );
        RegistryFriendlyByteBuf framedOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        protocolInfo.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);
        byte[] body = bytesAfterVarIntPrefix(framed);

        RegistryFriendlyByteBuf framedIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(framed), registryAccess);
        Packet<?> decodedPacket = protocolInfo.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundStoreCookiePacket decodedStoreCookie)) {
            throw new IllegalStateException(
                "decoded Play store_cookie as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "Identifier.parse(String); ClientboundStoreCookiePacket(Identifier, byte[]); ClientboundStoreCookiePacket.STREAM_CODEC; ClientboundStoreCookiePacket.PAYLOAD_STREAM_CODEC; ClientboundStoreCookiePacket.key(); ClientboundStoreCookiePacket.payload(); GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundStoreCookiePacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.common.ClientboundStoreCookiePacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:store_cookie",
            decodedPacket,
            "official ClientboundStoreCookiePacket fixture with Identifier key and byte[] payload",
            "Identifier followed by byte array encoded by PAYLOAD_STREAM_CODEC (max 5120 bytes)",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_key", key.toString());
        answerBody.put("stream_decoded_key", streamDecoded.key().toString());
        answerBody.put("decoded_key", decodedStoreCookie.key().toString());
        answerBody.put("input_payload_hex", HexFormat.of().formatHex(payload));
        answerBody.put("stream_decoded_payload_hex", HexFormat.of().formatHex(streamDecoded.payload()));
        answerBody.put("decoded_payload_hex", HexFormat.of().formatHex(decodedStoreCookie.payload()));
        answerBody.put("input_payload_length", payload.length);
        answerBody.put("stream_decoded_payload_length", streamDecoded.payload().length);
        answerBody.put("decoded_payload_length", decodedStoreCookie.payload().length);
        answerBody.put("stream_decoded_payload_equals_input", Arrays.equals(payload, streamDecoded.payload()));
        answerBody.put("decoded_payload_equals_input", Arrays.equals(payload, decodedStoreCookie.payload()));
        answer.put("answer", answerBody);
        return answer;
    }
}
