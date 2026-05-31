package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundTakeItemEntityPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;


public final class PlayTakeItemEntityClientboundFramedDispatchCase {
    private PlayTakeItemEntityClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        ClientboundTakeItemEntityPacket packet =
            new ClientboundTakeItemEntityPacket(
                inputFields.get("item_id").getAsInt(),
                inputFields.get("player_id").getAsInt(),
                inputFields.get("amount").getAsInt()
            );

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundTakeItemEntityPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundTakeItemEntityPacket streamDecoded =
            ClientboundTakeItemEntityPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:take_item_entity");

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
        if (!(decodedPacket instanceof ClientboundTakeItemEntityPacket decodedTakeItemEntity)) {
            throw new IllegalStateException(
                "decoded Play take_item_entity as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundTakeItemEntityPacket(int, int, int); ClientboundTakeItemEntityPacket.STREAM_CODEC; FriendlyByteBuf.readVarInt/writeVarInt x3; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundTakeItemEntityPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundTakeItemEntityPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:take_item_entity",
            decodedPacket,
            "official ClientboundTakeItemEntityPacket primitive ids/count fixture; no initialized entity or item stack state is required for codec proof",
            "itemId/playerId/amount as three VarInts through ClientboundTakeItemEntityPacket.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_item_id", packet.getItemId());
        answerBody.put("stream_decoded_item_id", streamDecoded.getItemId());
        answerBody.put("decoded_item_id", decodedTakeItemEntity.getItemId());
        answerBody.put("input_player_id", packet.getPlayerId());
        answerBody.put("stream_decoded_player_id", streamDecoded.getPlayerId());
        answerBody.put("decoded_player_id", decodedTakeItemEntity.getPlayerId());
        answerBody.put("input_amount", packet.getAmount());
        answerBody.put("stream_decoded_amount", streamDecoded.getAmount());
        answerBody.put("decoded_amount", decodedTakeItemEntity.getAmount());
        answer.put("answer", answerBody);
        return answer;
    }
}
