package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundForgetLevelChunkPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.level.ChunkPos;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlayForgetLevelChunkClientboundFramedDispatchCase {
    private PlayForgetLevelChunkClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        ChunkPos pos = new ChunkPos(inputFields.get("chunk_x").getAsInt(), inputFields.get("chunk_z").getAsInt());
        ClientboundForgetLevelChunkPacket packet = new ClientboundForgetLevelChunkPacket(pos);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundForgetLevelChunkPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundForgetLevelChunkPacket streamDecoded =
            ClientboundForgetLevelChunkPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:forget_level_chunk");

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
        if (!(decodedPacket instanceof ClientboundForgetLevelChunkPacket decodedForget)) {
            throw new IllegalStateException(
                "decoded Play forget_level_chunk as unexpected packet "
                    + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundForgetLevelChunkPacket(ChunkPos), ClientboundForgetLevelChunkPacket.STREAM_CODEC, FriendlyByteBuf.readChunkPos/writeChunkPos, ChunkPos(int, int), GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundForgetLevelChunkPacket), ClientboundForgetLevelChunkPacket.pos()",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundForgetLevelChunkPacket net.minecraft.world.level.ChunkPos net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:forget_level_chunk",
            decodedPacket,
            "official ClientboundForgetLevelChunkPacket ChunkPos constructor fixture; no initialized Level, chunk storage, or game state",
            "chunk position encoded by FriendlyByteBuf.writeChunkPos/readChunkPos through ClientboundForgetLevelChunkPacket.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_chunk_x", pos.x());
        answerBody.put("input_chunk_z", pos.z());
        answerBody.put("stream_decoded_chunk_x", streamDecoded.pos().x());
        answerBody.put("stream_decoded_chunk_z", streamDecoded.pos().z());
        answerBody.put("decoded_chunk_x", decodedForget.pos().x());
        answerBody.put("decoded_chunk_z", decodedForget.pos().z());
        answerBody.put("input_chunk_pos_packed", pos.pack());
        answerBody.put("decoded_chunk_pos_packed", decodedForget.pos().pack());
        answer.put("answer", answerBody);
        return answer;
    }
}
