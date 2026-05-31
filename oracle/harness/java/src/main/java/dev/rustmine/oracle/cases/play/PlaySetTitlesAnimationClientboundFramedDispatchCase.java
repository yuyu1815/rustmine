package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundSetTitlesAnimationPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;


public final class PlaySetTitlesAnimationClientboundFramedDispatchCase {
    private PlaySetTitlesAnimationClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        ClientboundSetTitlesAnimationPacket packet =
            new ClientboundSetTitlesAnimationPacket(
                inputFields.get("fade_in").getAsInt(),
                inputFields.get("stay").getAsInt(),
                inputFields.get("fade_out").getAsInt()
            );

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundSetTitlesAnimationPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundSetTitlesAnimationPacket streamDecoded =
            ClientboundSetTitlesAnimationPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:set_titles_animation");

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
        if (!(decodedPacket instanceof ClientboundSetTitlesAnimationPacket decodedTitlesAnimation)) {
            throw new IllegalStateException(
                "decoded Play set_titles_animation as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundSetTitlesAnimationPacket(int, int, int); ClientboundSetTitlesAnimationPacket.STREAM_CODEC; FriendlyByteBuf.readInt/writeInt x3; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundSetTitlesAnimationPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSetTitlesAnimationPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:set_titles_animation",
            decodedPacket,
            "official ClientboundSetTitlesAnimationPacket primitive timing fixture; no title text, UI screen, or client state is required",
            "fadeIn/stay/fadeOut as three big-endian FriendlyByteBuf int fields",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_fade_in", packet.getFadeIn());
        answerBody.put("stream_decoded_fade_in", streamDecoded.getFadeIn());
        answerBody.put("decoded_fade_in", decodedTitlesAnimation.getFadeIn());
        answerBody.put("input_stay", packet.getStay());
        answerBody.put("stream_decoded_stay", streamDecoded.getStay());
        answerBody.put("decoded_stay", decodedTitlesAnimation.getStay());
        answerBody.put("input_fade_out", packet.getFadeOut());
        answerBody.put("stream_decoded_fade_out", streamDecoded.getFadeOut());
        answerBody.put("decoded_fade_out", decodedTitlesAnimation.getFadeOut());
        answer.put("answer", answerBody);
        return answer;
    }
}
