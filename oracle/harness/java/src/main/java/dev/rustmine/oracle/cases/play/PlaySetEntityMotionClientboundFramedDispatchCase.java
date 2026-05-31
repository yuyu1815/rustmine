package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundSetEntityMotionPacket;
import net.minecraft.network.protocol.game.GameProtocols;
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


public final class PlaySetEntityMotionClientboundFramedDispatchCase {
    private PlaySetEntityMotionClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        Vec3 movement = new Vec3(
            inputFields.get("movement_x").getAsDouble(),
            inputFields.get("movement_y").getAsDouble(),
            inputFields.get("movement_z").getAsDouble()
        );
        ClientboundSetEntityMotionPacket packet = new ClientboundSetEntityMotionPacket(
            inputFields.get("entity_id").getAsInt(),
            movement
        );

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundSetEntityMotionPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundSetEntityMotionPacket streamDecoded =
            ClientboundSetEntityMotionPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:set_entity_motion");

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
        if (!(decodedPacket instanceof ClientboundSetEntityMotionPacket decodedMotion)) {
            throw new IllegalStateException(
                "decoded Play set_entity_motion as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundSetEntityMotionPacket(int, Vec3); ClientboundSetEntityMotionPacket.STREAM_CODEC; ByteBufCodecs.VAR_INT; Vec3.LP_STREAM_CODEC; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundSetEntityMotionPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSetEntityMotionPacket net.minecraft.world.phys.Vec3 net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:set_entity_motion",
            decodedPacket,
            "official ClientboundSetEntityMotionPacket primitive entity id plus Vec3 movement fixture; no entity instance, Level, or render state is required",
            "entity id VarInt followed by Vec3.LP_STREAM_CODEC movement through ClientboundSetEntityMotionPacket.STREAM_CODEC",
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
        answerBody.put("decoded_entity_id", decodedMotion.id());
        putMovement(answerBody, "input", packet.movement());
        putMovement(answerBody, "stream_decoded", streamDecoded.movement());
        putMovement(answerBody, "decoded", decodedMotion.movement());
        answer.put("answer", answerBody);
        return answer;
    }

    private static void putMovement(Map<String, Object> answerBody, String prefix, Vec3 movement) {
        answerBody.put(prefix + "_movement_x", movement.x);
        answerBody.put(prefix + "_movement_y", movement.y);
        answerBody.put(prefix + "_movement_z", movement.z);
    }
}
