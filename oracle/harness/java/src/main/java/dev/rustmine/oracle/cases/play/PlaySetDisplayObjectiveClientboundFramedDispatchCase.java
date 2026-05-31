package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundSetDisplayObjectivePacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.scores.DisplaySlot;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;


public final class PlaySetDisplayObjectiveClientboundFramedDispatchCase {
    private PlaySetDisplayObjectiveClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        DisplaySlot slot = DisplaySlot.valueOf(inputFields.get("slot").getAsString());
        ClientboundSetDisplayObjectivePacket packet =
            new ClientboundSetDisplayObjectivePacket(slot, null);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundSetDisplayObjectivePacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundSetDisplayObjectivePacket streamDecoded =
            ClientboundSetDisplayObjectivePacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:set_display_objective");

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
        if (!(decodedPacket instanceof ClientboundSetDisplayObjectivePacket decodedDisplayObjective)) {
            throw new IllegalStateException(
                "decoded Play set_display_objective as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundSetDisplayObjectivePacket(DisplaySlot, Objective) with null Objective; ClientboundSetDisplayObjectivePacket.STREAM_CODEC; FriendlyByteBuf.readById/writeById; FriendlyByteBuf.readUtf/writeUtf; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundSetDisplayObjectivePacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSetDisplayObjectivePacket net.minecraft.network.FriendlyByteBuf net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:set_display_objective",
            decodedPacket,
            "official ClientboundSetDisplayObjectivePacket clear-slot fixture using DisplaySlot and null Objective; no scoreboard Objective object, Component, or initialized scoreboard state is required",
            "display slot encoded with FriendlyByteBuf.writeById VarInt, followed by empty objective name string",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_display_slot", slot.name());
        answerBody.put("input_display_slot_id", slot.id());
        answerBody.put("stream_decoded_display_slot", streamDecoded.getSlot().name());
        answerBody.put("stream_decoded_display_slot_id", streamDecoded.getSlot().id());
        answerBody.put("decoded_display_slot", decodedDisplayObjective.getSlot().name());
        answerBody.put("decoded_display_slot_id", decodedDisplayObjective.getSlot().id());
        answerBody.put("input_objective_present", false);
        answerBody.put("stream_decoded_objective_present", streamDecoded.getObjectiveName() != null);
        answerBody.put("decoded_objective_present", decodedDisplayObjective.getObjectiveName() != null);
        answer.put("answer", answerBody);
        return answer;
    }
}
