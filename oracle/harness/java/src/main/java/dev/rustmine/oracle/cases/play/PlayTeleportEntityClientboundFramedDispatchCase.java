package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import java.util.Set;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundTeleportEntityPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.entity.PositionMoveRotation;
import net.minecraft.world.entity.Relative;
import net.minecraft.world.phys.Vec3;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;

public final class PlayTeleportEntityClientboundFramedDispatchCase {
    private PlayTeleportEntityClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int entityId = inputFields.get("entity_id").getAsInt();
        boolean onGround = inputFields.get("on_ground").getAsBoolean();
        PositionMoveRotation change = new PositionMoveRotation(
            new Vec3(1.25D, 2.5D, -3.75D),
            new Vec3(0.0D, 0.0D, 0.0D),
            45.0F,
            10.0F
        );
        ClientboundTeleportEntityPacket packet =
            ClientboundTeleportEntityPacket.teleport(entityId, change, Set.<Relative>of(), onGround);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundTeleportEntityPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundTeleportEntityPacket streamDecoded =
            ClientboundTeleportEntityPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:teleport_entity");

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
        if (!(decodedPacket instanceof ClientboundTeleportEntityPacket decodedTeleportEntity)) {
            throw new IllegalStateException(
                "decoded Play teleport_entity as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundTeleportEntityPacket.teleport(int, PositionMoveRotation, Set<Relative>, boolean); PositionMoveRotation.STREAM_CODEC; Relative.SET_STREAM_CODEC empty set; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundTeleportEntityPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundTeleportEntityPacket net.minecraft.world.entity.PositionMoveRotation net.minecraft.world.entity.Relative net.minecraft.world.phys.Vec3 net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:teleport_entity",
            decodedPacket,
            "official ClientboundTeleportEntityPacket.teleport primitive fixture with entity id, PositionMoveRotation values, empty relative set, and onGround=false; no entity instance is required",
            "entity id VarInt, position Vec3, delta Vec3, yRot float, xRot float, empty relative bitset int, onGround boolean",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_entity_id", packet.id());
        answerBody.put("stream_decoded_entity_id", streamDecoded.id());
        answerBody.put("decoded_entity_id", decodedTeleportEntity.id());
        answerBody.put("input_position_x", packet.change().position().x);
        answerBody.put("stream_decoded_position_x", streamDecoded.change().position().x);
        answerBody.put("decoded_position_x", decodedTeleportEntity.change().position().x);
        answerBody.put("input_position_y", packet.change().position().y);
        answerBody.put("stream_decoded_position_y", streamDecoded.change().position().y);
        answerBody.put("decoded_position_y", decodedTeleportEntity.change().position().y);
        answerBody.put("input_position_z", packet.change().position().z);
        answerBody.put("stream_decoded_position_z", streamDecoded.change().position().z);
        answerBody.put("decoded_position_z", decodedTeleportEntity.change().position().z);
        answerBody.put("input_relative_count", packet.relatives().size());
        answerBody.put("stream_decoded_relative_count", streamDecoded.relatives().size());
        answerBody.put("decoded_relative_count", decodedTeleportEntity.relatives().size());
        answerBody.put("input_on_ground", packet.onGround());
        answerBody.put("stream_decoded_on_ground", streamDecoded.onGround());
        answerBody.put("decoded_on_ground", decodedTeleportEntity.onGround());
        answer.put("answer", answerBody);
        return answer;
    }
}
