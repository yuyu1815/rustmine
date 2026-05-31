package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundTrackedWaypointPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;

public final class PlayWaypointClientboundFramedDispatchCase {
    private PlayWaypointClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        UUID waypointId = UUID.fromString(
            input.getAsJsonObject("question").getAsJsonObject("input_fields").get("waypoint_uuid").getAsString()
        );
        ClientboundTrackedWaypointPacket packet = ClientboundTrackedWaypointPacket.removeWaypoint(waypointId);

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundTrackedWaypointPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundTrackedWaypointPacket streamDecoded =
            ClientboundTrackedWaypointPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:waypoint");

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
        if (!(decodedPacket instanceof ClientboundTrackedWaypointPacket decodedWaypoint)) {
            throw new IllegalStateException(
                "decoded Play waypoint as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundTrackedWaypointPacket.removeWaypoint(UUID); TrackedWaypoint.empty(UUID); ClientboundTrackedWaypointPacket.Operation.UNTRACK; TrackedWaypoint.STREAM_CODEC empty waypoint; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundTrackedWaypointPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundTrackedWaypointPacket net.minecraft.network.protocol.game.ClientboundTrackedWaypointPacket\\$Operation net.minecraft.world.waypoints.TrackedWaypoint net.minecraft.world.waypoints.TrackedWaypoint\\$EmptyWaypoint net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:waypoint",
            decodedPacket,
            "official ClientboundTrackedWaypointPacket.removeWaypoint(UUID) fixture; operation is UNTRACK and waypoint is TrackedWaypoint.empty(UUID), so no position/chunk/azimuth/icon semantics are entered",
            "operation enum id for UNTRACK followed by empty tracked-waypoint identifier/type payload",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        Enum<?> inputOperation = (Enum<?>) packet.operation();
        Enum<?> streamDecodedOperation = (Enum<?>) streamDecoded.operation();
        Enum<?> decodedOperation = (Enum<?>) decodedWaypoint.operation();
        answerBody.put("input_waypoint_uuid", waypointId.toString());
        answerBody.put("input_operation", inputOperation.name());
        answerBody.put("stream_decoded_operation", streamDecodedOperation.name());
        answerBody.put("decoded_operation", decodedOperation.name());
        answerBody.put("decoded_operation_ordinal", decodedOperation.ordinal());
        answerBody.put("input_waypoint_id", packet.waypoint().id().toString());
        answerBody.put("stream_decoded_waypoint_id", streamDecoded.waypoint().id().toString());
        answerBody.put("decoded_waypoint_id", decodedWaypoint.waypoint().id().toString());
        answer.put("answer", answerBody);
        return answer;
    }
}
