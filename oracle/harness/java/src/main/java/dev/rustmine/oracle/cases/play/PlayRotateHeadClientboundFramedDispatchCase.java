package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundRotateHeadPacket;
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


public final class PlayRotateHeadClientboundFramedDispatchCase {
    private PlayRotateHeadClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int entityId = inputFields.get("entity_id").getAsInt();
        byte yHeadRot = (byte) inputFields.get("y_head_rot_byte").getAsInt();

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        fixtureBodyOut.writeVarInt(entityId);
        fixtureBodyOut.writeByte(yHeadRot);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundRotateHeadPacket packet = ClientboundRotateHeadPacket.STREAM_CODEC.decode(packetIn);
        FriendlyByteBuf streamIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundRotateHeadPacket streamDecoded = ClientboundRotateHeadPacket.STREAM_CODEC.decode(streamIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:rotate_head");

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
        if (!(decodedPacket instanceof ClientboundRotateHeadPacket decodedRotateHead)) {
            throw new IllegalStateException(
                "decoded Play rotate_head as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundRotateHeadPacket.STREAM_CODEC; private ClientboundRotateHeadPacket(FriendlyByteBuf); private write(FriendlyByteBuf); FriendlyByteBuf.readVarInt/writeVarInt; FriendlyByteBuf.readByte/writeByte; ClientboundRotateHeadPacket.getYHeadRot(); GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundRotateHeadPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundRotateHeadPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:rotate_head",
            decodedPacket,
            "official ClientboundRotateHeadPacket STREAM_CODEC decode fixture from primitive entity id and head-rotation byte; no initialized Entity or Level state is required",
            "entity id VarInt followed by signed yHeadRot byte through ClientboundRotateHeadPacket.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_entity_id", entityId);
        answerBody.put("stream_decoded_entity_id", privateInt(streamDecoded, "entityId"));
        answerBody.put("decoded_entity_id", privateInt(decodedRotateHead, "entityId"));
        answerBody.put("input_y_head_rot_byte", (int) yHeadRot);
        answerBody.put("stream_decoded_y_head_rot_byte", (int) privateByte(streamDecoded, "yHeadRot"));
        answerBody.put("decoded_y_head_rot_byte", (int) privateByte(decodedRotateHead, "yHeadRot"));
        answerBody.put("stream_decoded_y_head_rot_degrees", streamDecoded.getYHeadRot());
        answerBody.put("decoded_y_head_rot_degrees", decodedRotateHead.getYHeadRot());
        answer.put("answer", answerBody);
        return answer;
    }
}
