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
import net.minecraft.network.protocol.game.ClientboundMoveEntityPacket;
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


public final class PlayMoveEntityPosRotClientboundFramedDispatchCase {
    private PlayMoveEntityPosRotClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        ClientboundMoveEntityPacket.PosRot packet = new ClientboundMoveEntityPacket.PosRot(
            inputFields.get("entity_id").getAsInt(),
            inputFields.get("xa").getAsShort(),
            inputFields.get("ya").getAsShort(),
            inputFields.get("za").getAsShort(),
            inputFields.get("y_rot").getAsByte(),
            inputFields.get("x_rot").getAsByte(),
            inputFields.get("on_ground").getAsBoolean()
        );

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundMoveEntityPacket.PosRot.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundMoveEntityPacket.PosRot streamDecoded =
            ClientboundMoveEntityPacket.PosRot.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:move_entity_pos_rot");

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
        if (!(decodedPacket instanceof ClientboundMoveEntityPacket.PosRot decodedMove)) {
            throw new IllegalStateException(
                "decoded Play move_entity_pos_rot as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundMoveEntityPacket.PosRot(int, short, short, short, byte, byte, boolean), ClientboundMoveEntityPacket.PosRot.STREAM_CODEC, FriendlyByteBuf.readVarInt/writeVarInt, readShort/writeShort, readByte/writeByte, readBoolean/writeBoolean, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundMoveEntityPacket.PosRot), ClientGamePacketListener.handleMoveEntity(ClientboundMoveEntityPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundMoveEntityPacket 'net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$PosRot' net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:move_entity_pos_rot",
            decodedPacket,
            "official ClientboundMoveEntityPacket.PosRot primitive entity id/delta/rotation/onGround fixture; no initialized Entity, Level, or game state",
            "entity id VarInt, three signed short deltas, yRot byte, xRot byte, and onGround boolean through ClientboundMoveEntityPacket.PosRot.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        putMoveEntityFields(answerBody, "input", packet);
        putMoveEntityFields(answerBody, "stream_decoded", streamDecoded);
        putMoveEntityFields(answerBody, "decoded", decodedMove);
        answer.put("answer", answerBody);
        return answer;
    }
}
