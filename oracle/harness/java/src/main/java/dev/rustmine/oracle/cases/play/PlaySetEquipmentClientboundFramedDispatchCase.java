package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import com.mojang.datafixers.util.Pair;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundSetEquipmentPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.entity.EquipmentSlot;
import net.minecraft.world.item.ItemStack;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;

public final class PlaySetEquipmentClientboundFramedDispatchCase {
    private PlaySetEquipmentClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int entityId = inputFields.get("entity_id").getAsInt();
        EquipmentSlot slot = EquipmentSlot.MAINHAND;
        ItemStack itemStack = ItemStack.EMPTY;
        ClientboundSetEquipmentPacket packet = new ClientboundSetEquipmentPacket(
            entityId,
            List.of(Pair.of(slot, itemStack))
        );
        RegistryAccess registryAccess = RegistryAccess.EMPTY;

        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundSetEquipmentPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundSetEquipmentPacket streamDecoded =
            ClientboundSetEquipmentPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:set_equipment");

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
        if (!(decodedPacket instanceof ClientboundSetEquipmentPacket decodedEquipment)) {
            throw new IllegalStateException(
                "decoded Play set_equipment as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        var streamDecodedEntry = streamDecoded.getSlots().get(0);
        var decodedEntry = decodedEquipment.getSlots().get(0);
        Map<String, Object> answer = playAnswerHeader(
            input,
            "ItemStack.EMPTY, EquipmentSlot.MAINHAND, ClientboundSetEquipmentPacket(int, List<Pair<EquipmentSlot, ItemStack>>), ClientboundSetEquipmentPacket.STREAM_CODEC, ClientboundSetEquipmentPacket.write/decode, ItemStack.OPTIONAL_STREAM_CODEC, ItemStack$1.encode/decode, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundSetEquipmentPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSetEquipmentPacket 'net.minecraft.world.item.ItemStack$1' net.minecraft.world.item.ItemStack net.minecraft.world.entity.EquipmentSlot net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:set_equipment",
            decodedPacket,
            "official ClientboundSetEquipmentPacket(entityId, one MAINHAND ItemStack.EMPTY entry) fixture; no initialized entity, equipment state, item registry entry, component registry, or game state",
            "entity id VarInt, one equipment slot byte without continuation mask, and ItemStack.OPTIONAL_STREAM_CODEC empty stack VarInt count marker 0",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_entity_id", entityId);
        answerBody.put("stream_decoded_entity_id", streamDecoded.getEntity());
        answerBody.put("decoded_entity_id", decodedEquipment.getEntity());
        answerBody.put("input_equipment_slot", slot.name());
        answerBody.put("stream_decoded_equipment_slot", streamDecodedEntry.getFirst().name());
        answerBody.put("decoded_equipment_slot", decodedEntry.getFirst().name());
        answerBody.put("input_equipment_slot_ordinal", slot.ordinal());
        answerBody.put("stream_decoded_equipment_slot_ordinal", streamDecodedEntry.getFirst().ordinal());
        answerBody.put("decoded_equipment_slot_ordinal", decodedEntry.getFirst().ordinal());
        answerBody.put("input_equipment_entry_count", 1);
        answerBody.put("stream_decoded_equipment_entry_count", streamDecoded.getSlots().size());
        answerBody.put("decoded_equipment_entry_count", decodedEquipment.getSlots().size());
        answerBody.put("input_item_empty", itemStack.isEmpty());
        answerBody.put("stream_decoded_item_empty", streamDecodedEntry.getSecond().isEmpty());
        answerBody.put("decoded_item_empty", decodedEntry.getSecond().isEmpty());
        answer.put("answer", answerBody);
        return answer;
    }
}
