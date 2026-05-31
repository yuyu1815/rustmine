package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.Map;
import java.util.List;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.common.ClientboundCustomReportDetailsPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;


public final class PlayCustomReportDetailsClientboundFramedDispatchCase {
    private PlayCustomReportDetailsClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        Map<String, String> details = Map.of();
        ClientboundCustomReportDetailsPacket packet = new ClientboundCustomReportDetailsPacket(details);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundCustomReportDetailsPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundCustomReportDetailsPacket streamDecoded =
            ClientboundCustomReportDetailsPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:custom_report_details");

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
        if (!(decodedPacket instanceof ClientboundCustomReportDetailsPacket decodedCustomReportDetails)) {
            throw new IllegalStateException(
                "decoded Play custom_report_details as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundCustomReportDetailsPacket(Map<String, String>), ClientboundCustomReportDetailsPacket.STREAM_CODEC, ClientboundCustomReportDetailsPacket.details(), ByteBufCodecs.map(..., maxCount=32), GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundCustomReportDetailsPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.common.ClientboundCustomReportDetailsPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:custom_report_details",
            decodedPacket,
            "official ClientboundCustomReportDetailsPacket empty Map.of() fixture",
            "zero-length details map; no key/value strings follow",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_fixture", "Map.of() details");
        answerBody.put("input_details", details);
        answerBody.put("stream_decoded_details", streamDecoded.details());
        answerBody.put("decoded_details", decodedCustomReportDetails.details());
        answerBody.put("input_detail_count", details.size());
        answerBody.put("stream_decoded_detail_count", streamDecoded.details().size());
        answerBody.put("decoded_detail_count", decodedCustomReportDetails.details().size());
        answer.put("answer", answerBody);
        return answer;
    }
}
