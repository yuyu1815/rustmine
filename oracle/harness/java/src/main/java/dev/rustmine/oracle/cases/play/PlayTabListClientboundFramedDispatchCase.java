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
import net.minecraft.network.protocol.game.ClientboundTabListPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;

public final class PlayTabListClientboundFramedDispatchCase {
    private PlayTabListClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        Component header = Component.literal(inputFields.get("header").getAsString());
        Component footer = Component.literal(inputFields.get("footer").getAsString());
        ClientboundTabListPacket packet = new ClientboundTabListPacket(header, footer);

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundTabListPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundTabListPacket streamDecoded =
            ClientboundTabListPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:tab_list");

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
        if (!(decodedPacket instanceof ClientboundTabListPacket decodedTabList)) {
            throw new IllegalStateException(
                "decoded Play tab_list as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "Component.literal(String), ClientboundTabListPacket(Component, Component), ClientboundTabListPacket.STREAM_CODEC, ComponentSerialization.TRUSTED_STREAM_CODEC x2, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundTabListPacket), ClientboundTabListPacket.header(), ClientboundTabListPacket.footer(), Component.getString()",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundTabListPacket net.minecraft.network.chat.ComponentSerialization net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:tab_list",
            decodedPacket,
            "official Play ClientboundTabListPacket Component.literal header/footer fixture; no initialized player-list UI, client, level, registry contents, or game state",
            "header and footer encoded by ComponentSerialization.TRUSTED_STREAM_CODEC through ClientboundTabListPacket.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("header_component_fixture", "Component.literal(\"" + header.getString() + "\")");
        answerBody.put("footer_component_fixture", "Component.literal(\"" + footer.getString() + "\")");
        answerBody.put("input_header_text", header.getString());
        answerBody.put("stream_decoded_header_text", streamDecoded.header().getString());
        answerBody.put("decoded_header_text", decodedTabList.header().getString());
        answerBody.put("input_footer_text", footer.getString());
        answerBody.put("stream_decoded_footer_text", streamDecoded.footer().getString());
        answerBody.put("decoded_footer_text", decodedTabList.footer().getString());
        answer.put("answer", answerBody);
        return answer;
    }
}
