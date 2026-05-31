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
import net.minecraft.network.protocol.game.ClientboundContainerSetContentPacket;
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


public final class PlayContainerSetContentClientboundFramedDispatchCase {
    private PlayContainerSetContentClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int containerId = inputFields.get("container_id").getAsInt();
        int stateId = inputFields.get("state_id").getAsInt();
        List<ItemStack> items = List.of();
        ItemStack carriedItem = ItemStack.EMPTY;
        ClientboundContainerSetContentPacket packet =
            new ClientboundContainerSetContentPacket(containerId, stateId, items, carriedItem);

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundContainerSetContentPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundContainerSetContentPacket streamDecoded =
            ClientboundContainerSetContentPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] containerSetContentPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((packetType, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", packetType.id().toString());
            row.put("flow", packetType.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:container_set_content".equals(packetType.id().toString())) {
                containerSetContentPacketId[0] = packetId;
            }
        });
        if (containerSetContentPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound container_set_content packet id");
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
        if (!(decodedPacket instanceof ClientboundContainerSetContentPacket decodedSetContent)) {
            throw new IllegalStateException(
                "decoded Play container_set_content as unexpected packet " + decodedPacket.getClass().getName()
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
            "function_or_member", "ClientboundContainerSetContentPacket(int, int, List<ItemStack>, ItemStack), ClientboundContainerSetContentPacket.STREAM_CODEC, ByteBufCodecs.CONTAINER_ID, ByteBufCodecs.VAR_INT, ItemStack.OPTIONAL_LIST_STREAM_CODEC, ItemStack.OPTIONAL_STREAM_CODEC, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...), ClientGamePacketListener.handleContainerContent(ClientboundContainerSetContentPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundContainerSetContentPacket 'net.minecraft.world.item.ItemStack$1' net.minecraft.world.item.ItemStack net.minecraft.network.codec.ByteBufCodecs net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.ClientGamePacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:container_set_content");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundContainerSetContentPacket(int, int, List<ItemStack>, ItemStack) constructor fixture with an empty item list and ItemStack.EMPTY carried item; no initialized Menu, screen, Level, inventory, item registry entry, or game state");
        answerBody.put("official_body_shape", "containerId encoded by ByteBufCodecs.CONTAINER_ID, stateId encoded by ByteBufCodecs.VAR_INT, items encoded by ItemStack.OPTIONAL_LIST_STREAM_CODEC as a VarInt list length followed by optional ItemStack entries, and carriedItem encoded by ItemStack.OPTIONAL_STREAM_CODEC; this fixture uses list length 0 and empty carried stack");
        answerBody.put("input_container_id", containerId);
        answerBody.put("stream_decoded_container_id", streamDecoded.containerId());
        answerBody.put("decoded_container_id", decodedSetContent.containerId());
        answerBody.put("input_state_id", stateId);
        answerBody.put("stream_decoded_state_id", streamDecoded.stateId());
        answerBody.put("decoded_state_id", decodedSetContent.stateId());
        answerBody.put("input_item_count", items.size());
        answerBody.put("stream_decoded_item_count", streamDecoded.items().size());
        answerBody.put("decoded_item_count", decodedSetContent.items().size());
        answerBody.put("input_carried_item_empty", carriedItem.isEmpty());
        answerBody.put("stream_decoded_carried_item_empty", streamDecoded.carriedItem().isEmpty());
        answerBody.put("decoded_carried_item_empty", decodedSetContent.carriedItem().isEmpty());
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
