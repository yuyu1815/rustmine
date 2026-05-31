package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.time.Instant;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundContainerSetSlotPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.item.ItemStack;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlayContainerSetSlotClientboundFramedDispatchCase {
    private PlayContainerSetSlotClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int containerId = inputFields.get("container_id").getAsInt();
        int stateId = inputFields.get("state_id").getAsInt();
        int slot = inputFields.get("slot").getAsInt();
        ItemStack itemStack = ItemStack.EMPTY;
        ClientboundContainerSetSlotPacket packet =
            new ClientboundContainerSetSlotPacket(containerId, stateId, slot, itemStack);

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundContainerSetSlotPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundContainerSetSlotPacket streamDecoded =
            ClientboundContainerSetSlotPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] containerSetSlotPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((packetType, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", packetType.id().toString());
            row.put("flow", packetType.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:container_set_slot".equals(packetType.id().toString())) {
                containerSetSlotPacketId[0] = packetId;
            }
        });
        if (containerSetSlotPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound container_set_slot packet id");
        }

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
        if (!(decodedPacket instanceof ClientboundContainerSetSlotPacket decodedSetSlot)) {
            throw new IllegalStateException(
                "decoded Play container_set_slot as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundContainerSetSlotPacket(int, int, int, ItemStack), ClientboundContainerSetSlotPacket.STREAM_CODEC, ClientboundContainerSetSlotPacket(RegistryFriendlyByteBuf), ClientboundContainerSetSlotPacket.write(RegistryFriendlyByteBuf), RegistryFriendlyByteBuf.readContainerId/writeContainerId, RegistryFriendlyByteBuf.readVarInt/writeVarInt, RegistryFriendlyByteBuf.readShort/writeShort, ItemStack.OPTIONAL_STREAM_CODEC, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...), ClientGamePacketListener.handleContainerSetSlot(ClientboundContainerSetSlotPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundContainerSetSlotPacket net.minecraft.world.item.ItemStack net.minecraft.network.codec.ByteBufCodecs net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.ClientGamePacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:container_set_slot");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundContainerSetSlotPacket(int, int, int, ItemStack) constructor fixture with ItemStack.EMPTY; no initialized Menu, screen, Level, inventory, item registry entry, component registry, or game state");
        answerBody.put("official_body_shape", "containerId encoded by RegistryFriendlyByteBuf.writeContainerId/readContainerId, stateId encoded by RegistryFriendlyByteBuf.writeVarInt/readVarInt, slot encoded by RegistryFriendlyByteBuf.writeShort/readShort, and itemStack encoded by ItemStack.OPTIONAL_STREAM_CODEC; this fixture uses the empty stack marker");
        answerBody.put("input_container_id", containerId);
        answerBody.put("stream_decoded_container_id", streamDecoded.getContainerId());
        answerBody.put("decoded_container_id", decodedSetSlot.getContainerId());
        answerBody.put("input_state_id", stateId);
        answerBody.put("stream_decoded_state_id", streamDecoded.getStateId());
        answerBody.put("decoded_state_id", decodedSetSlot.getStateId());
        answerBody.put("input_slot", slot);
        answerBody.put("stream_decoded_slot", streamDecoded.getSlot());
        answerBody.put("decoded_slot", decodedSetSlot.getSlot());
        answerBody.put("input_item_empty", itemStack.isEmpty());
        answerBody.put("stream_decoded_item_empty", streamDecoded.getItem().isEmpty());
        answerBody.put("decoded_item_empty", decodedSetSlot.getItem().isEmpty());
        answerBody.put("remaining_after_packet_stream_decode", packetIn.readableBytes());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("fixture_body_hex", HexFormat.of().formatHex(fixtureBody));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("play_clientbound_packet_table", playClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }
}
