package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundStartConfigurationPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;


public final class PlayStartConfigurationClientboundFramedDispatchCase {
    private PlayStartConfigurationClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        ClientboundStartConfigurationPacket packet = ClientboundStartConfigurationPacket.INSTANCE;

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundStartConfigurationPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundStartConfigurationPacket streamDecoded =
            ClientboundStartConfigurationPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:start_configuration");

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
        if (!(decodedPacket instanceof ClientboundStartConfigurationPacket decodedStartConfiguration)) {
            throw new IllegalStateException(
                "decoded Play start_configuration as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundStartConfigurationPacket.INSTANCE; ClientboundStartConfigurationPacket.STREAM_CODEC; StreamCodec.unit(INSTANCE); ClientboundStartConfigurationPacket.isTerminal(); GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundStartConfigurationPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundStartConfigurationPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:start_configuration",
            decodedPacket,
            "official ClientboundStartConfigurationPacket singleton unit fixture; empty packet body with terminal flag",
            "StreamCodec.unit(INSTANCE) empty body",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("stream_decoded_same_instance", streamDecoded == ClientboundStartConfigurationPacket.INSTANCE);
        answerBody.put("decoded_same_instance", decodedStartConfiguration == ClientboundStartConfigurationPacket.INSTANCE);
        answerBody.put("input_is_terminal", packet.isTerminal());
        answerBody.put("decoded_is_terminal", decodedStartConfiguration.isTerminal());
        answer.put("answer", answerBody);
        return answer;
    }
}
