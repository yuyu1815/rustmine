package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundTagQueryPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;

public final class PlayTagQueryClientboundFramedDispatchCase {
    private PlayTagQueryClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int transactionId = inputFields.get("transaction_id").getAsInt();
        CompoundTag tag = new CompoundTag();
        ClientboundTagQueryPacket packet = new ClientboundTagQueryPacket(transactionId, tag);

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundTagQueryPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundTagQueryPacket streamDecoded =
            ClientboundTagQueryPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:tag_query");

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
        if (!(decodedPacket instanceof ClientboundTagQueryPacket decodedTagQuery)) {
            throw new IllegalStateException(
                "decoded Play tag_query as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundTagQueryPacket(int, new CompoundTag()), ClientboundTagQueryPacket.write/STREAM_CODEC, FriendlyByteBuf.writeVarInt, FriendlyByteBuf.writeNbt, FriendlyByteBuf.readNbt, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundTagQueryPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundTagQueryPacket net.minecraft.network.FriendlyByteBuf net.minecraft.nbt.CompoundTag net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:tag_query",
            decodedPacket,
            "official ClientboundTagQueryPacket(transactionId, new CompoundTag()) fixture; empty compound avoids named NBT payload semantics",
            "transaction id VarInt followed by FriendlyByteBuf.writeNbt empty compound root",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_transaction_id", transactionId);
        answerBody.put("stream_decoded_transaction_id", streamDecoded.getTransactionId());
        answerBody.put("decoded_transaction_id", decodedTagQuery.getTransactionId());
        answerBody.put("input_tag_size", tag.size());
        answerBody.put("stream_decoded_tag_size", streamDecoded.getTag().size());
        answerBody.put("decoded_tag_size", decodedTagQuery.getTag().size());
        answerBody.put("input_tag_snbt", tag.toString());
        answerBody.put("stream_decoded_tag_snbt", streamDecoded.getTag().toString());
        answerBody.put("decoded_tag_snbt", decodedTagQuery.getTag().toString());
        answer.put("answer", answerBody);
        return answer;
    }
}
