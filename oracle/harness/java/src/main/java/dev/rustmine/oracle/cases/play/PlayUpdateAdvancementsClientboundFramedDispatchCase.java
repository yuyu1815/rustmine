package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import java.util.Set;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundUpdateAdvancementsPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;

public final class PlayUpdateAdvancementsClientboundFramedDispatchCase {
    private PlayUpdateAdvancementsClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        boolean reset = input.getAsJsonObject("question").getAsJsonObject("input_fields").get("reset").getAsBoolean();
        boolean showAdvancements =
            input.getAsJsonObject("question").getAsJsonObject("input_fields").get("show_advancements").getAsBoolean();
        ClientboundUpdateAdvancementsPacket packet =
            new ClientboundUpdateAdvancementsPacket(reset, List.of(), Set.of(), Map.of(), showAdvancements);

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundUpdateAdvancementsPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundUpdateAdvancementsPacket streamDecoded =
            ClientboundUpdateAdvancementsPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:update_advancements");

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
        if (!(decodedPacket instanceof ClientboundUpdateAdvancementsPacket decodedUpdateAdvancements)) {
            throw new IllegalStateException(
                "decoded Play update_advancements as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundUpdateAdvancementsPacket(boolean, Collection<AdvancementHolder>, Set<Identifier>, Map<Identifier, AdvancementProgress>, boolean); ClientboundUpdateAdvancementsPacket.STREAM_CODEC; empty added/removed/progress collections; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundUpdateAdvancementsPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundUpdateAdvancementsPacket net.minecraft.advancements.AdvancementHolder net.minecraft.advancements.AdvancementProgress net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:update_advancements",
            decodedPacket,
            "official ClientboundUpdateAdvancementsPacket fixture with reset=false, showAdvancements=false, and empty added/removed/progress collections; no advancement holder or progress payload is entered",
            "reset boolean, empty added advancement list, empty removed identifier set, empty progress map, showAdvancements boolean",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_reset", packet.shouldReset());
        answerBody.put("stream_decoded_reset", streamDecoded.shouldReset());
        answerBody.put("decoded_reset", decodedUpdateAdvancements.shouldReset());
        answerBody.put("input_show_advancements", packet.shouldShowAdvancements());
        answerBody.put("stream_decoded_show_advancements", streamDecoded.shouldShowAdvancements());
        answerBody.put("decoded_show_advancements", decodedUpdateAdvancements.shouldShowAdvancements());
        answerBody.put("input_added_count", packet.getAdded().size());
        answerBody.put("stream_decoded_added_count", streamDecoded.getAdded().size());
        answerBody.put("decoded_added_count", decodedUpdateAdvancements.getAdded().size());
        answerBody.put("input_removed_count", packet.getRemoved().size());
        answerBody.put("stream_decoded_removed_count", streamDecoded.getRemoved().size());
        answerBody.put("decoded_removed_count", decodedUpdateAdvancements.getRemoved().size());
        answerBody.put("input_progress_count", packet.getProgress().size());
        answerBody.put("stream_decoded_progress_count", streamDecoded.getProgress().size());
        answerBody.put("decoded_progress_count", decodedUpdateAdvancements.getProgress().size());
        answer.put("answer", answerBody);
        return answer;
    }
}
