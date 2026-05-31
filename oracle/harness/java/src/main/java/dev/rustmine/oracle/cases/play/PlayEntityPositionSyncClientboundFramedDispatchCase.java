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
import net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.entity.PositionMoveRotation;
import net.minecraft.world.phys.Vec3;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlayEntityPositionSyncClientboundFramedDispatchCase {
    private PlayEntityPositionSyncClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int entityId = inputFields.get("entity_id").getAsInt();
        Vec3 position = new Vec3(
            inputFields.get("x").getAsDouble(),
            inputFields.get("y").getAsDouble(),
            inputFields.get("z").getAsDouble()
        );
        Vec3 deltaMovement = new Vec3(
            inputFields.get("delta_x").getAsDouble(),
            inputFields.get("delta_y").getAsDouble(),
            inputFields.get("delta_z").getAsDouble()
        );
        float yRot = inputFields.get("y_rot").getAsFloat();
        float xRot = inputFields.get("x_rot").getAsFloat();
        boolean onGround = inputFields.get("on_ground").getAsBoolean();
        PositionMoveRotation values = new PositionMoveRotation(position, deltaMovement, yRot, xRot);
        ClientboundEntityPositionSyncPacket packet =
            new ClientboundEntityPositionSyncPacket(entityId, values, onGround);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundEntityPositionSyncPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundEntityPositionSyncPacket streamDecoded =
            ClientboundEntityPositionSyncPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:entity_position_sync");

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
        if (!(decodedPacket instanceof ClientboundEntityPositionSyncPacket decodedSync)) {
            throw new IllegalStateException(
                "decoded Play entity_position_sync as unexpected packet "
                    + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundEntityPositionSyncPacket(int, PositionMoveRotation, boolean), ClientboundEntityPositionSyncPacket.STREAM_CODEC, PositionMoveRotation.STREAM_CODEC, Vec3.STREAM_CODEC, ByteBufCodecs.VAR_INT, ByteBufCodecs.BOOL, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundEntityPositionSyncPacket), ClientboundEntityPositionSyncPacket.id(), values(), onGround()",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket net.minecraft.world.entity.PositionMoveRotation net.minecraft.world.phys.Vec3 net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:entity_position_sync",
            decodedPacket,
            "official ClientboundEntityPositionSyncPacket constructor fixture with primitive id, position, movement, rotation, and onGround values; no initialized Entity, Level, or game state",
            "entity id VarInt, PositionMoveRotation as Vec3 position, Vec3 deltaMovement, yRot float, xRot float, then onGround boolean through ClientboundEntityPositionSyncPacket.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        putEntityPositionSyncFields(answerBody, "input", packet);
        putEntityPositionSyncFields(answerBody, "stream_decoded", streamDecoded);
        putEntityPositionSyncFields(answerBody, "decoded", decodedSync);
        answer.put("answer", answerBody);
        return answer;
    }
}
