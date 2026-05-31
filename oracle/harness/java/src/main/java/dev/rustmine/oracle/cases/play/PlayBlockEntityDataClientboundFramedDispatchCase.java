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
import net.minecraft.network.protocol.game.ClientboundBlockEntityDataPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.core.registries.Registries;
import net.minecraft.world.level.block.entity.BlockEntityType;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlayBlockEntityDataClientboundFramedDispatchCase {
    private PlayBlockEntityDataClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int blockX = inputFields.get("block_x").getAsInt();
        int blockY = inputFields.get("block_y").getAsInt();
        int blockZ = inputFields.get("block_z").getAsInt();
        String expectedBlockEntityType = inputFields.get("block_entity_type").getAsString();
        int expectedTagSize = inputFields.get("tag_size").getAsInt();
        BlockEntityType<?> type = BlockEntityType.CHEST;
        String blockEntityType = BuiltInRegistries.BLOCK_ENTITY_TYPE.getKey(type).toString();
        if (!expectedBlockEntityType.equals(blockEntityType)) {
            throw new IllegalArgumentException(
                "minimal block_entity_data fixture expected " + expectedBlockEntityType
                    + " but official type is " + blockEntityType
            );
        }
        BlockPos pos = new BlockPos(blockX, blockY, blockZ);
        CompoundTag tag = new CompoundTag();
        if (expectedTagSize != tag.size()) {
            throw new IllegalArgumentException("minimal block_entity_data fixture only supports empty tag");
        }
        ClientboundBlockEntityDataPacket packet =
            constructBlockEntityDataPacket(pos, type, tag);
        RegistryAccess registryAccess =
            RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY);

        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundBlockEntityDataPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundBlockEntityDataPacket streamDecoded =
            ClientboundBlockEntityDataPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] blockEntityDataPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((packetType, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", packetType.id().toString());
            row.put("flow", packetType.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:block_entity_data".equals(packetType.id().toString())) {
                blockEntityDataPacketId[0] = packetId;
            }
        });
        if (blockEntityDataPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound block_entity_data packet id");
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
        if (!(decodedPacket instanceof ClientboundBlockEntityDataPacket decodedBlockEntityData)) {
            throw new IllegalStateException(
                "decoded Play block_entity_data as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        BlockPos streamDecodedPos = streamDecoded.getPos();
        BlockPos decodedPos = decodedBlockEntityData.getPos();
        String streamDecodedType = BuiltInRegistries.BLOCK_ENTITY_TYPE
            .getKey(streamDecoded.getType())
            .toString();
        String decodedType = BuiltInRegistries.BLOCK_ENTITY_TYPE
            .getKey(decodedBlockEntityData.getType())
            .toString();
        int typeRegistryId = BuiltInRegistries.BLOCK_ENTITY_TYPE.getId(type);

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
            "function_or_member", "private ClientboundBlockEntityDataPacket(BlockPos, BlockEntityType<?>, CompoundTag), ClientboundBlockEntityDataPacket.STREAM_CODEC, BlockPos.STREAM_CODEC, ByteBufCodecs.registry(Registries.BLOCK_ENTITY_TYPE), ByteBufCodecs.TRUSTED_COMPOUND_TAG, BuiltInRegistries.BLOCK_ENTITY_TYPE, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY))).codec().encode/decode(...), ClientGamePacketListener.handleBlockEntityData(ClientboundBlockEntityDataPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p -verbose net.minecraft.network.protocol.game.ClientboundBlockEntityDataPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.world.level.block.entity.BlockEntityType net.minecraft.core.registries.BuiltInRegistries net.minecraft.nbt.CompoundTag"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:block_entity_data");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official private ClientboundBlockEntityDataPacket BlockPos, built-in BlockEntityType.CHEST, and empty CompoundTag constructor fixture; requires bootstrapped built-in registries but no initialized Level, BlockEntity, or game state");
        answerBody.put("official_body_shape", "block position encoded with BlockPos.STREAM_CODEC, block entity type encoded with ByteBufCodecs.registry(Registries.BLOCK_ENTITY_TYPE), and tag encoded with ByteBufCodecs.TRUSTED_COMPOUND_TAG");
        answerBody.put("input_block_x", blockX);
        answerBody.put("input_block_y", blockY);
        answerBody.put("input_block_z", blockZ);
        answerBody.put("stream_decoded_block_x", streamDecodedPos.getX());
        answerBody.put("stream_decoded_block_y", streamDecodedPos.getY());
        answerBody.put("stream_decoded_block_z", streamDecodedPos.getZ());
        answerBody.put("decoded_block_x", decodedPos.getX());
        answerBody.put("decoded_block_y", decodedPos.getY());
        answerBody.put("decoded_block_z", decodedPos.getZ());
        answerBody.put("input_block_entity_type", blockEntityType);
        answerBody.put("stream_decoded_block_entity_type", streamDecodedType);
        answerBody.put("decoded_block_entity_type", decodedType);
        answerBody.put("decoded_block_entity_type_registry_id", typeRegistryId);
        answerBody.put("input_tag_size", tag.size());
        answerBody.put("stream_decoded_tag_size", streamDecoded.getTag().size());
        answerBody.put("decoded_tag_size", decodedBlockEntityData.getTag().size());
        answerBody.put("input_tag_snbt", tag.toString());
        answerBody.put("stream_decoded_tag_snbt", streamDecoded.getTag().toString());
        answerBody.put("decoded_tag_snbt", decodedBlockEntityData.getTag().toString());
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
