package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundSelectAdvancementsTabPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.resources.Identifier;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlaySelectAdvancementsTabClientboundFramedDispatchCase {
    private PlaySelectAdvancementsTabClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        Identifier tab = Identifier.parse(inputFields.get("tab").getAsString());
        ClientboundSelectAdvancementsTabPacket packet = new ClientboundSelectAdvancementsTabPacket(tab);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundSelectAdvancementsTabPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundSelectAdvancementsTabPacket streamDecoded =
            ClientboundSelectAdvancementsTabPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:select_advancements_tab");

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
        if (!(decodedPacket instanceof ClientboundSelectAdvancementsTabPacket decodedSelectTab)) {
            throw new IllegalStateException(
                "decoded Play select_advancements_tab as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "Identifier.parse(String); ClientboundSelectAdvancementsTabPacket(Identifier); ClientboundSelectAdvancementsTabPacket.STREAM_CODEC; FriendlyByteBuf.readNullable/writeNullable(Identifier); GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundSelectAdvancementsTabPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSelectAdvancementsTabPacket net.minecraft.resources.Identifier net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:select_advancements_tab",
            decodedPacket,
            "official ClientboundSelectAdvancementsTabPacket non-null Identifier fixture; no advancement tree, screen state, or initialized client state is required",
            "nullable Identifier marker plus Identifier string through ClientboundSelectAdvancementsTabPacket.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_tab", tab.toString());
        answerBody.put("stream_decoded_tab", streamDecoded.getTab().toString());
        answerBody.put("decoded_tab", decodedSelectTab.getTab().toString());
        answer.put("answer", answerBody);
        return answer;
    }
}
