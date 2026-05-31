package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundSetCursorItemPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.item.ItemStack;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;

public final class PlaySetCursorItemClientboundFramedDispatchCase {
    private PlaySetCursorItemClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        ItemStack contents = ItemStack.EMPTY;
        ClientboundSetCursorItemPacket packet = new ClientboundSetCursorItemPacket(contents);
        RegistryAccess registryAccess = RegistryAccess.EMPTY;

        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundSetCursorItemPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundSetCursorItemPacket streamDecoded =
            ClientboundSetCursorItemPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:set_cursor_item");

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
        if (!(decodedPacket instanceof ClientboundSetCursorItemPacket decodedCursorItem)) {
            throw new IllegalStateException(
                "decoded Play set_cursor_item as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ItemStack.EMPTY, ClientboundSetCursorItemPacket(ItemStack), ClientboundSetCursorItemPacket.STREAM_CODEC, ItemStack.OPTIONAL_STREAM_CODEC, ItemStack$1.encode/decode, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundSetCursorItemPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSetCursorItemPacket 'net.minecraft.world.item.ItemStack$1' net.minecraft.world.item.ItemStack net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:set_cursor_item",
            decodedPacket,
            "official ClientboundSetCursorItemPacket(ItemStack.EMPTY) fixture; no initialized inventory, item registry entry, component registry, or game state",
            "contents encoded by ItemStack.OPTIONAL_STREAM_CODEC; this fixture uses the empty stack VarInt count marker 0",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_item_empty", contents.isEmpty());
        answerBody.put("stream_decoded_item_empty", streamDecoded.contents().isEmpty());
        answerBody.put("decoded_item_empty", decodedCursorItem.contents().isEmpty());
        answer.put("answer", answerBody);
        return answer;
    }
}
