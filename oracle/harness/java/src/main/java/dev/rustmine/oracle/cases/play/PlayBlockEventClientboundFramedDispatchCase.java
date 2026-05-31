package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.time.Instant;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.core.BlockPos;
import net.minecraft.core.RegistryAccess;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundBlockEventPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.core.registries.Registries;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.Blocks;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlayBlockEventClientboundFramedDispatchCase {
    private PlayBlockEventClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int blockX = inputFields.get("block_x").getAsInt();
        int blockY = inputFields.get("block_y").getAsInt();
        int blockZ = inputFields.get("block_z").getAsInt();
        String expectedBlock = inputFields.get("block").getAsString();
        int eventType = inputFields.get("event_type").getAsInt();
        int eventData = inputFields.get("event_data").getAsInt();
        Block block = Blocks.NOTE_BLOCK;
        String blockName = BuiltInRegistries.BLOCK.getKey(block).toString();
        if (!expectedBlock.equals(blockName)) {
            throw new IllegalArgumentException(
                "minimal block_event fixture expected " + expectedBlock
                    + " but official block is " + blockName
            );
        }
        BlockPos pos = new BlockPos(blockX, blockY, blockZ);
        ClientboundBlockEventPacket packet =
            new ClientboundBlockEventPacket(pos, block, eventType, eventData);
        RegistryAccess registryAccess =
            RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY);

        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundBlockEventPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundBlockEventPacket streamDecoded =
            ClientboundBlockEventPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] blockEventPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((packetType, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", packetType.id().toString());
            row.put("flow", packetType.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:block_event".equals(packetType.id().toString())) {
                blockEventPacketId[0] = packetId;
            }
        });
        if (blockEventPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound block_event packet id");
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
        if (!(decodedPacket instanceof ClientboundBlockEventPacket decodedBlockEvent)) {
            throw new IllegalStateException(
                "decoded Play block_event as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        BlockPos streamDecodedPos = streamDecoded.getPos();
        BlockPos decodedPos = decodedBlockEvent.getPos();
        String streamDecodedBlock = BuiltInRegistries.BLOCK
            .getKey(streamDecoded.getBlock())
            .toString();
        String decodedBlock = BuiltInRegistries.BLOCK
            .getKey(decodedBlockEvent.getBlock())
            .toString();
        int blockRegistryId = BuiltInRegistries.BLOCK.getId(block);

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
            "function_or_member", "ClientboundBlockEventPacket(BlockPos, Block, int, int), ClientboundBlockEventPacket.STREAM_CODEC, private ClientboundBlockEventPacket(RegistryFriendlyByteBuf), private write(RegistryFriendlyByteBuf), RegistryFriendlyByteBuf.readBlockPos/writeBlockPos, RegistryFriendlyByteBuf.readUnsignedByte/writeByte, ByteBufCodecs.registry(Registries.BLOCK), BuiltInRegistries.BLOCK, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY))).codec().encode/decode(...), ClientGamePacketListener.handleBlockEvent(ClientboundBlockEventPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p -verbose net.minecraft.network.protocol.game.ClientboundBlockEventPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.world.level.block.Block net.minecraft.world.level.block.Blocks net.minecraft.core.registries.BuiltInRegistries"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:block_event");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundBlockEventPacket BlockPos, built-in Blocks.NOTE_BLOCK, event type, and event data constructor fixture; requires bootstrapped built-in registries but no initialized Level, BlockEntity, or game state");
        answerBody.put("official_body_shape", "block position encoded with RegistryFriendlyByteBuf BlockPos, event type encoded as one unsigned byte, event data encoded as one unsigned byte, and block encoded with ByteBufCodecs.registry(Registries.BLOCK)");
        answerBody.put("input_block_x", blockX);
        answerBody.put("input_block_y", blockY);
        answerBody.put("input_block_z", blockZ);
        answerBody.put("stream_decoded_block_x", streamDecodedPos.getX());
        answerBody.put("stream_decoded_block_y", streamDecodedPos.getY());
        answerBody.put("stream_decoded_block_z", streamDecodedPos.getZ());
        answerBody.put("decoded_block_x", decodedPos.getX());
        answerBody.put("decoded_block_y", decodedPos.getY());
        answerBody.put("decoded_block_z", decodedPos.getZ());
        answerBody.put("input_event_type", eventType);
        answerBody.put("stream_decoded_event_type", streamDecoded.getB0());
        answerBody.put("decoded_event_type", decodedBlockEvent.getB0());
        answerBody.put("input_event_data", eventData);
        answerBody.put("stream_decoded_event_data", streamDecoded.getB1());
        answerBody.put("decoded_event_data", decodedBlockEvent.getB1());
        answerBody.put("input_block", blockName);
        answerBody.put("stream_decoded_block", streamDecodedBlock);
        answerBody.put("decoded_block", decodedBlock);
        answerBody.put("decoded_block_registry_id", blockRegistryId);
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
