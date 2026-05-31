package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundSetBorderLerpSizePacket;
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


public final class PlaySetBorderLerpSizeClientboundFramedDispatchCase {
    private PlaySetBorderLerpSizeClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        WorldBorder border = new WorldBorder();
        border.lerpSizeBetween(
            inputFields.get("old_size").getAsDouble(),
            inputFields.get("new_size").getAsDouble(),
            inputFields.get("lerp_time").getAsLong(),
            0L
        );
        ClientboundSetBorderLerpSizePacket packet = new ClientboundSetBorderLerpSizePacket(border);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundSetBorderLerpSizePacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundSetBorderLerpSizePacket streamDecoded =
            ClientboundSetBorderLerpSizePacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:set_border_lerp_size");

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
        if (!(decodedPacket instanceof ClientboundSetBorderLerpSizePacket decodedBorderLerpSize)) {
            throw new IllegalStateException(
                "decoded Play set_border_lerp_size as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "WorldBorder(); WorldBorder.lerpSizeBetween(double, double, long, long); ClientboundSetBorderLerpSizePacket(WorldBorder); ClientboundSetBorderLerpSizePacket.STREAM_CODEC; FriendlyByteBuf.readDouble/writeDouble; FriendlyByteBuf.readVarLong/writeVarLong; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundSetBorderLerpSizePacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSetBorderLerpSizePacket net.minecraft.world.level.border.WorldBorder 'net.minecraft.world.level.border.WorldBorder$MovingBorderExtent' net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:set_border_lerp_size",
            decodedPacket,
            "official ClientboundSetBorderLerpSizePacket primitive WorldBorder lerp fixture; no initialized Level or world-border runtime state is required",
            "old size double, new size double, then lerp time VarLong through ClientboundSetBorderLerpSizePacket.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_old_size", packet.getOldSize());
        answerBody.put("stream_decoded_old_size", streamDecoded.getOldSize());
        answerBody.put("decoded_old_size", decodedBorderLerpSize.getOldSize());
        answerBody.put("input_new_size", packet.getNewSize());
        answerBody.put("stream_decoded_new_size", streamDecoded.getNewSize());
        answerBody.put("decoded_new_size", decodedBorderLerpSize.getNewSize());
        answerBody.put("input_lerp_time", packet.getLerpTime());
        answerBody.put("stream_decoded_lerp_time", streamDecoded.getLerpTime());
        answerBody.put("decoded_lerp_time", decodedBorderLerpSize.getLerpTime());
        answer.put("answer", answerBody);
        return answer;
    }
}
