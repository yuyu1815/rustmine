package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundSetDefaultSpawnPositionPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.level.storage.LevelData;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;

public final class PlaySetDefaultSpawnPositionClientboundFramedDispatchCase {
    private PlaySetDefaultSpawnPositionClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        ClientboundSetDefaultSpawnPositionPacket packet =
            new ClientboundSetDefaultSpawnPositionPacket(LevelData.RespawnData.DEFAULT);

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundSetDefaultSpawnPositionPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundSetDefaultSpawnPositionPacket streamDecoded =
            ClientboundSetDefaultSpawnPositionPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:set_default_spawn_position");

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
        if (!(decodedPacket instanceof ClientboundSetDefaultSpawnPositionPacket decodedSpawn)) {
            throw new IllegalStateException(
                "decoded Play set_default_spawn_position as unexpected packet "
                    + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundSetDefaultSpawnPositionPacket(LevelData.RespawnData.DEFAULT), LevelData.RespawnData.STREAM_CODEC, GlobalPos.STREAM_CODEC, ByteBufCodecs.FLOAT, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundSetDefaultSpawnPositionPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSetDefaultSpawnPositionPacket net.minecraft.world.level.storage.LevelData\\$RespawnData net.minecraft.core.GlobalPos net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:set_default_spawn_position",
            decodedPacket,
            "official ClientboundSetDefaultSpawnPositionPacket(LevelData.RespawnData.DEFAULT) fixture; proves only default overworld zero BlockPos/yaw/pitch body, not respawn behavior",
            "LevelData.RespawnData.STREAM_CODEC: GlobalPos dimension ResourceKey plus BlockPos, then yaw float and pitch float",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_dimension", packet.respawnData().dimension().identifier().toString());
        answerBody.put("stream_decoded_dimension", streamDecoded.respawnData().dimension().identifier().toString());
        answerBody.put("decoded_dimension", decodedSpawn.respawnData().dimension().identifier().toString());
        answerBody.put("input_block_x", packet.respawnData().pos().getX());
        answerBody.put("input_block_y", packet.respawnData().pos().getY());
        answerBody.put("input_block_z", packet.respawnData().pos().getZ());
        answerBody.put("stream_decoded_block_x", streamDecoded.respawnData().pos().getX());
        answerBody.put("stream_decoded_block_y", streamDecoded.respawnData().pos().getY());
        answerBody.put("stream_decoded_block_z", streamDecoded.respawnData().pos().getZ());
        answerBody.put("decoded_block_x", decodedSpawn.respawnData().pos().getX());
        answerBody.put("decoded_block_y", decodedSpawn.respawnData().pos().getY());
        answerBody.put("decoded_block_z", decodedSpawn.respawnData().pos().getZ());
        answerBody.put("input_yaw", packet.respawnData().yaw());
        answerBody.put("stream_decoded_yaw", streamDecoded.respawnData().yaw());
        answerBody.put("decoded_yaw", decodedSpawn.respawnData().yaw());
        answerBody.put("input_pitch", packet.respawnData().pitch());
        answerBody.put("stream_decoded_pitch", streamDecoded.respawnData().pitch());
        answerBody.put("decoded_pitch", decodedSpawn.respawnData().pitch());
        answer.put("answer", answerBody);
        return answer;
    }
}
