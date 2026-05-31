package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundTestInstanceBlockStatus;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;

public final class PlayTestInstanceBlockStatusClientboundFramedDispatchCase {
    private PlayTestInstanceBlockStatusClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        Component status = Component.literal(inputFields.get("status").getAsString());
        ClientboundTestInstanceBlockStatus packet =
            new ClientboundTestInstanceBlockStatus(status, Optional.empty());

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundTestInstanceBlockStatus.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundTestInstanceBlockStatus streamDecoded =
            ClientboundTestInstanceBlockStatus.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:test_instance_block_status");

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
        if (!(decodedPacket instanceof ClientboundTestInstanceBlockStatus decodedStatus)) {
            throw new IllegalStateException(
                "decoded Play test_instance_block_status as unexpected packet "
                    + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "Component.literal(String), ClientboundTestInstanceBlockStatus(Component, Optional.empty()), ClientboundTestInstanceBlockStatus.STREAM_CODEC, ComponentSerialization.STREAM_CODEC, ByteBufCodecs.optional(Vec3i.STREAM_CODEC) absent branch, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundTestInstanceBlockStatus)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundTestInstanceBlockStatus net.minecraft.network.chat.ComponentSerialization net.minecraft.core.Vec3i net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:test_instance_block_status",
            decodedPacket,
            "official ClientboundTestInstanceBlockStatus(Component.literal, Optional.empty()) fixture; absent size avoids Vec3i/game-test block semantics",
            "status Component encoded by ComponentSerialization.STREAM_CODEC followed by false optional Vec3i marker",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("component_fixture", "Component.literal(\"" + status.getString() + "\")");
        answerBody.put("input_component_text", status.getString());
        answerBody.put("stream_decoded_component_text", streamDecoded.status().getString());
        answerBody.put("decoded_component_text", decodedStatus.status().getString());
        answerBody.put("input_size_present", packet.size().isPresent());
        answerBody.put("stream_decoded_size_present", streamDecoded.size().isPresent());
        answerBody.put("decoded_size_present", decodedStatus.size().isPresent());
        answer.put("answer", answerBody);
        return answer;
    }
}
