package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundSetHealthPacket;
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


public final class PlaySetHealthClientboundFramedDispatchCase {
    private PlaySetHealthClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        ClientboundSetHealthPacket packet = new ClientboundSetHealthPacket(
            inputFields.get("health").getAsFloat(),
            inputFields.get("food").getAsInt(),
            inputFields.get("saturation").getAsFloat()
        );

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundSetHealthPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundSetHealthPacket streamDecoded =
            ClientboundSetHealthPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:set_health");

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
        if (!(decodedPacket instanceof ClientboundSetHealthPacket decodedHealth)) {
            throw new IllegalStateException(
                "decoded Play set_health as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundSetHealthPacket(float, int, float); ClientboundSetHealthPacket.STREAM_CODEC; FriendlyByteBuf.readFloat/writeFloat health; FriendlyByteBuf.readVarInt/writeVarInt food; FriendlyByteBuf.readFloat/writeFloat saturation; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundSetHealthPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSetHealthPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:set_health",
            decodedPacket,
            "official ClientboundSetHealthPacket primitive health/food/saturation fixture; no player state is required",
            "health float, food VarInt, saturation float through ClientboundSetHealthPacket.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        putHealth(answerBody, "input", packet);
        putHealth(answerBody, "stream_decoded", streamDecoded);
        putHealth(answerBody, "decoded", decodedHealth);
        answer.put("answer", answerBody);
        return answer;
    }

    private static void putHealth(Map<String, Object> answerBody, String prefix, ClientboundSetHealthPacket packet) {
        answerBody.put(prefix + "_health", packet.getHealth());
        answerBody.put(prefix + "_food", packet.getFood());
        answerBody.put(prefix + "_saturation", packet.getSaturation());
    }
}
