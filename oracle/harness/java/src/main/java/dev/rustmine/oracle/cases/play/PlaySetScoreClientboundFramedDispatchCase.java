package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundSetScorePacket;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;


public final class PlaySetScoreClientboundFramedDispatchCase {
    private PlaySetScoreClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String owner = inputFields.get("owner").getAsString();
        String objectiveName = inputFields.get("objective_name").getAsString();
        int score = inputFields.get("score").getAsInt();
        ClientboundSetScorePacket packet = new ClientboundSetScorePacket(
            owner,
            objectiveName,
            score,
            Optional.empty(),
            Optional.empty()
        );

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundSetScorePacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundSetScorePacket streamDecoded =
            ClientboundSetScorePacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:set_score");

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
        if (!(decodedPacket instanceof ClientboundSetScorePacket decodedScore)) {
            throw new IllegalStateException(
                "decoded Play set_score as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundSetScorePacket(String, String, int, Optional.empty(), Optional.empty()); ClientboundSetScorePacket.STREAM_CODEC; ByteBufCodecs.STRING_UTF8; ByteBufCodecs.VAR_INT; ComponentSerialization.TRUSTED_OPTIONAL_STREAM_CODEC absent branch; NumberFormatTypes.OPTIONAL_STREAM_CODEC absent branch; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundSetScorePacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSetScorePacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:set_score",
            decodedPacket,
            "official ClientboundSetScorePacket scoreboard update fixture with plain owner/objective strings, VarInt score, and absent optional display/number format",
            "owner string, objectiveName string, score VarInt, absent optional Component marker, absent optional number-format marker",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_owner", owner);
        answerBody.put("stream_decoded_owner", streamDecoded.owner());
        answerBody.put("decoded_owner", decodedScore.owner());
        answerBody.put("input_objective_name", objectiveName);
        answerBody.put("stream_decoded_objective_name", streamDecoded.objectiveName());
        answerBody.put("decoded_objective_name", decodedScore.objectiveName());
        answerBody.put("input_score", score);
        answerBody.put("stream_decoded_score", streamDecoded.score());
        answerBody.put("decoded_score", decodedScore.score());
        answerBody.put("input_display_present", packet.display().isPresent());
        answerBody.put("stream_decoded_display_present", streamDecoded.display().isPresent());
        answerBody.put("decoded_display_present", decodedScore.display().isPresent());
        answerBody.put("input_number_format_present", packet.numberFormat().isPresent());
        answerBody.put("stream_decoded_number_format_present", streamDecoded.numberFormat().isPresent());
        answerBody.put("decoded_number_format_present", decodedScore.numberFormat().isPresent());
        answer.put("answer", answerBody);
        return answer;
    }
}
