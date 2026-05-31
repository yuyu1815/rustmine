package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundSetObjectivePacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.scores.Objective;
import net.minecraft.world.scores.Scoreboard;
import net.minecraft.world.scores.criteria.ObjectiveCriteria;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;

public final class PlaySetObjectiveClientboundFramedDispatchCase {
    private PlaySetObjectiveClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String objectiveName = inputFields.get("objective_name").getAsString();
        Objective objective = new Objective(
            new Scoreboard(),
            objectiveName,
            ObjectiveCriteria.DUMMY,
            Component.literal(objectiveName),
            ObjectiveCriteria.RenderType.INTEGER,
            false,
            null
        );
        ClientboundSetObjectivePacket packet =
            new ClientboundSetObjectivePacket(objective, ClientboundSetObjectivePacket.METHOD_REMOVE);

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundSetObjectivePacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundSetObjectivePacket streamDecoded =
            ClientboundSetObjectivePacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:set_objective");

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
        if (!(decodedPacket instanceof ClientboundSetObjectivePacket decodedObjective)) {
            throw new IllegalStateException(
                "decoded Play set_objective as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "Objective(Scoreboard, String, ObjectiveCriteria.DUMMY, Component.literal, RenderType.INTEGER, false, null), ClientboundSetObjectivePacket(Objective, METHOD_REMOVE), ClientboundSetObjectivePacket.write/STREAM_CODEC remove branch writes only objectiveName and method byte, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundSetObjectivePacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSetObjectivePacket net.minecraft.world.scores.Objective net.minecraft.world.scores.Scoreboard net.minecraft.world.scores.criteria.ObjectiveCriteria net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:set_objective",
            decodedPacket,
            "official ClientboundSetObjectivePacket(Objective, METHOD_REMOVE) fixture; remove branch serializes only objective name and method byte",
            "objective name String followed by signed method byte 1; display Component, render type, and number-format are absent on the remove branch",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_objective_name", objectiveName);
        answerBody.put("stream_decoded_objective_name", streamDecoded.getObjectiveName());
        answerBody.put("decoded_objective_name", decodedObjective.getObjectiveName());
        answerBody.put("input_method", ClientboundSetObjectivePacket.METHOD_REMOVE);
        answerBody.put("stream_decoded_method", streamDecoded.getMethod());
        answerBody.put("decoded_method", decodedObjective.getMethod());
        answerBody.put("input_display_present", false);
        answer.put("answer", answerBody);
        return answer;
    }
}
