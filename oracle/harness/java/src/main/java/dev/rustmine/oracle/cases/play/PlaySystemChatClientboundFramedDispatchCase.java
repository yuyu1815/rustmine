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
import net.minecraft.network.protocol.game.ClientboundSystemChatPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;

public final class PlaySystemChatClientboundFramedDispatchCase {
    private PlaySystemChatClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        Component content = Component.literal(inputFields.get("content").getAsString());
        boolean overlay = inputFields.get("overlay").getAsBoolean();
        ClientboundSystemChatPacket packet = new ClientboundSystemChatPacket(content, overlay);

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundSystemChatPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundSystemChatPacket streamDecoded =
            ClientboundSystemChatPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:system_chat");

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
        if (!(decodedPacket instanceof ClientboundSystemChatPacket decodedSystemChat)) {
            throw new IllegalStateException(
                "decoded Play system_chat as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "Component.literal(String), ClientboundSystemChatPacket(Component, boolean), ClientboundSystemChatPacket.STREAM_CODEC, ComponentSerialization.TRUSTED_STREAM_CODEC, ByteBufCodecs.BOOL, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundSystemChatPacket), ClientboundSystemChatPacket.content(), ClientboundSystemChatPacket.overlay(), Component.getString()",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSystemChatPacket net.minecraft.network.chat.ComponentSerialization net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:system_chat",
            decodedPacket,
            "official Play ClientboundSystemChatPacket Component.literal content plus false overlay fixture; no initialized chat HUD, client, level, registry contents, or game state",
            "content encoded by ComponentSerialization.TRUSTED_STREAM_CODEC followed by overlay boolean through ClientboundSystemChatPacket.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("component_fixture", "Component.literal(\"" + content.getString() + "\")");
        answerBody.put("input_component_text", content.getString());
        answerBody.put("stream_decoded_component_text", streamDecoded.content().getString());
        answerBody.put("decoded_component_text", decodedSystemChat.content().getString());
        answerBody.put("input_overlay", overlay);
        answerBody.put("stream_decoded_overlay", streamDecoded.overlay());
        answerBody.put("decoded_overlay", decodedSystemChat.overlay());
        answer.put("answer", answerBody);
        return answer;
    }
}
