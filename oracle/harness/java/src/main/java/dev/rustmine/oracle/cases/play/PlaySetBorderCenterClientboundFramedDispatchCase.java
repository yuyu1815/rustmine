package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundSetBorderCenterPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.level.border.WorldBorder;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlaySetBorderCenterClientboundFramedDispatchCase {
    private PlaySetBorderCenterClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        WorldBorder border = new WorldBorder();
        border.setCenter(
            inputFields.get("new_center_x").getAsDouble(),
            inputFields.get("new_center_z").getAsDouble()
        );
        ClientboundSetBorderCenterPacket packet = new ClientboundSetBorderCenterPacket(border);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundSetBorderCenterPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundSetBorderCenterPacket streamDecoded =
            ClientboundSetBorderCenterPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:set_border_center");

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
        if (!(decodedPacket instanceof ClientboundSetBorderCenterPacket decodedBorderCenter)) {
            throw new IllegalStateException(
                "decoded Play set_border_center as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "WorldBorder(); WorldBorder.setCenter(double, double); ClientboundSetBorderCenterPacket(WorldBorder); ClientboundSetBorderCenterPacket.STREAM_CODEC; FriendlyByteBuf.readDouble/writeDouble; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundSetBorderCenterPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSetBorderCenterPacket net.minecraft.world.level.border.WorldBorder net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:set_border_center",
            decodedPacket,
            "official ClientboundSetBorderCenterPacket primitive WorldBorder center fixture; no initialized Level or world-border runtime state is required",
            "new center X double followed by new center Z double through ClientboundSetBorderCenterPacket.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_new_center_x", packet.getNewCenterX());
        answerBody.put("stream_decoded_new_center_x", streamDecoded.getNewCenterX());
        answerBody.put("decoded_new_center_x", decodedBorderCenter.getNewCenterX());
        answerBody.put("input_new_center_z", packet.getNewCenterZ());
        answerBody.put("stream_decoded_new_center_z", streamDecoded.getNewCenterZ());
        answerBody.put("decoded_new_center_z", decodedBorderCenter.getNewCenterZ());
        answer.put("answer", answerBody);
        return answer;
    }
}
