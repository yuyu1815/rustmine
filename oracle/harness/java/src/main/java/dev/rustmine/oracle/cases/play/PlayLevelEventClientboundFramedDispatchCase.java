package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.BlockPos;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundLevelEventPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlayLevelEventClientboundFramedDispatchCase {
    private PlayLevelEventClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int type = inputFields.get("type").getAsInt();
        int blockX = inputFields.get("block_x").getAsInt();
        int blockY = inputFields.get("block_y").getAsInt();
        int blockZ = inputFields.get("block_z").getAsInt();
        int data = inputFields.get("data").getAsInt();
        boolean globalEvent = inputFields.get("global_event").getAsBoolean();
        BlockPos pos = new BlockPos(blockX, blockY, blockZ);
        ClientboundLevelEventPacket packet =
            new ClientboundLevelEventPacket(type, pos, data, globalEvent);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundLevelEventPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundLevelEventPacket streamDecoded =
            ClientboundLevelEventPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:level_event");

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
        Packet<? super ClientGamePacketListener> decodedPacket =
            protocolInfo.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundLevelEventPacket decodedLevelEvent)) {
            throw new IllegalStateException(
                "decoded Play level_event as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundLevelEventPacket(int, BlockPos, int, boolean), ClientboundLevelEventPacket.STREAM_CODEC, private ClientboundLevelEventPacket(FriendlyByteBuf), private write(FriendlyByteBuf), FriendlyByteBuf.readInt/writeInt, readBlockPos/writeBlockPos, readBoolean/writeBoolean, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundLevelEventPacket), ClientboundLevelEventPacket.getType(), getPos(), getData(), isGlobalEvent()",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundLevelEventPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:level_event",
            decodedPacket,
            "official ClientboundLevelEventPacket primitive type, BlockPos, data, and global flag constructor fixture; no initialized Level, block event handler, sound, particle, or game state",
            "type int, BlockPos, data int, and global event boolean through ClientboundLevelEventPacket.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        putLevelEventFields(answerBody, "input", packet);
        putLevelEventFields(answerBody, "stream_decoded", streamDecoded);
        putLevelEventFields(answerBody, "decoded", decodedLevelEvent);
        answer.put("answer", answerBody);
        return answer;
    }
}
