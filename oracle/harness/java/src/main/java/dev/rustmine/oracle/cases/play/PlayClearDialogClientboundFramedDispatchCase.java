package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.common.ClientboundClearDialogPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;


public final class PlayClearDialogClientboundFramedDispatchCase {
    private PlayClearDialogClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        ClientboundClearDialogPacket packet = ClientboundClearDialogPacket.INSTANCE;

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundClearDialogPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundClearDialogPacket streamDecoded =
            ClientboundClearDialogPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:clear_dialog");

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
        if (!(decodedPacket instanceof ClientboundClearDialogPacket decodedClearDialog)) {
            throw new IllegalStateException(
                "decoded Play clear_dialog as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundClearDialogPacket.INSTANCE; ClientboundClearDialogPacket.STREAM_CODEC; StreamCodec.unit(INSTANCE); GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundClearDialogPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.common.ClientboundClearDialogPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:clear_dialog",
            decodedPacket,
            "official ClientboundClearDialogPacket.INSTANCE singleton fixture",
            "StreamCodec.unit(INSTANCE) empty body",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_fixture", "ClientboundClearDialogPacket.INSTANCE");
        answerBody.put("stream_decoded_same_instance", streamDecoded == ClientboundClearDialogPacket.INSTANCE);
        answerBody.put("decoded_equals_instance", decodedClearDialog == ClientboundClearDialogPacket.INSTANCE);
        answer.put("answer", answerBody);
        return answer;
    }
}
