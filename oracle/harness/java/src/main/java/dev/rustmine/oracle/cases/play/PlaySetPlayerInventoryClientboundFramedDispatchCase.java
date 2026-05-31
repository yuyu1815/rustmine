package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundSetPlayerInventoryPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.item.ItemStack;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;

public final class PlaySetPlayerInventoryClientboundFramedDispatchCase {
    private PlaySetPlayerInventoryClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int slot = inputFields.get("slot").getAsInt();
        ItemStack contents = ItemStack.EMPTY;
        ClientboundSetPlayerInventoryPacket packet =
            new ClientboundSetPlayerInventoryPacket(slot, contents);
        RegistryAccess registryAccess = RegistryAccess.EMPTY;

        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundSetPlayerInventoryPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundSetPlayerInventoryPacket streamDecoded =
            ClientboundSetPlayerInventoryPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:set_player_inventory");

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
        if (!(decodedPacket instanceof ClientboundSetPlayerInventoryPacket decodedInventory)) {
            throw new IllegalStateException(
                "decoded Play set_player_inventory as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ItemStack.EMPTY, ClientboundSetPlayerInventoryPacket(int, ItemStack), ClientboundSetPlayerInventoryPacket.STREAM_CODEC, ByteBufCodecs.VAR_INT, ItemStack.OPTIONAL_STREAM_CODEC, ItemStack$1.encode/decode, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundSetPlayerInventoryPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSetPlayerInventoryPacket 'net.minecraft.world.item.ItemStack$1' net.minecraft.world.item.ItemStack net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:set_player_inventory",
            decodedPacket,
            "official ClientboundSetPlayerInventoryPacket(slot, ItemStack.EMPTY) fixture; no initialized inventory, item registry entry, component registry, or game state",
            "slot VarInt followed by ItemStack.OPTIONAL_STREAM_CODEC empty stack VarInt count marker 0",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_slot", slot);
        answerBody.put("stream_decoded_slot", streamDecoded.slot());
        answerBody.put("decoded_slot", decodedInventory.slot());
        answerBody.put("input_item_empty", contents.isEmpty());
        answerBody.put("stream_decoded_item_empty", streamDecoded.contents().isEmpty());
        answerBody.put("decoded_item_empty", decodedInventory.contents().isEmpty());
        answer.put("answer", answerBody);
        return answer;
    }
}
