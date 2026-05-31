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
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundBlockDestructionPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlayBlockDestructionClientboundFramedDispatchCase {
    private PlayBlockDestructionClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int breakerId = inputFields.get("breaker_id").getAsInt();
        int blockX = inputFields.get("block_x").getAsInt();
        int blockY = inputFields.get("block_y").getAsInt();
        int blockZ = inputFields.get("block_z").getAsInt();
        int progress = inputFields.get("progress").getAsInt();
        BlockPos pos = new BlockPos(blockX, blockY, blockZ);
        ClientboundBlockDestructionPacket packet =
            new ClientboundBlockDestructionPacket(breakerId, pos, progress);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundBlockDestructionPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundBlockDestructionPacket streamDecoded =
            ClientboundBlockDestructionPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] blockDestructionPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:block_destruction".equals(type.id().toString())) {
                blockDestructionPacketId[0] = packetId;
            }
        });
        if (blockDestructionPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound block_destruction packet id");
        }

        var protocolInfo = GameProtocols.CLIENTBOUND_TEMPLATE.bind(
            RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)
        );
        RegistryFriendlyByteBuf framedOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), RegistryAccess.EMPTY);
        protocolInfo.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);
        byte[] body = bytesAfterVarIntPrefix(framed);

        RegistryFriendlyByteBuf framedIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(framed), RegistryAccess.EMPTY);
        Packet<? super ClientGamePacketListener> decodedPacket =
            protocolInfo.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundBlockDestructionPacket decodedBlockDestruction)) {
            throw new IllegalStateException(
                "decoded Play block_destruction as unexpected packet " + decodedPacket.getClass().getName()
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
            "function_or_member", "ClientboundBlockDestructionPacket(int, BlockPos, int), ClientboundBlockDestructionPacket.STREAM_CODEC, private ClientboundBlockDestructionPacket(FriendlyByteBuf), private write(FriendlyByteBuf), FriendlyByteBuf.readVarInt/writeVarInt, FriendlyByteBuf.readBlockPos/writeBlockPos, FriendlyByteBuf.readUnsignedByte/writeByte, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...), ClientGamePacketListener.handleBlockDestruction(ClientboundBlockDestructionPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p -verbose net.minecraft.network.protocol.game.ClientboundBlockDestructionPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener"
        ));

        BlockPos streamDecodedPos = streamDecoded.getPos();
        BlockPos decodedPos = decodedBlockDestruction.getPos();
        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:block_destruction");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundBlockDestructionPacket breaker id, BlockPos, and progress constructor fixture; no initialized Minecraft/game state");
        answerBody.put("official_body_shape", "breaker id encoded as FriendlyByteBuf VarInt, block position encoded with FriendlyByteBuf BlockPos, progress encoded as one unsigned byte");
        answerBody.put("input_breaker_id", breakerId);
        answerBody.put("stream_decoded_breaker_id", streamDecoded.getId());
        answerBody.put("decoded_breaker_id", decodedBlockDestruction.getId());
        answerBody.put("input_block_x", blockX);
        answerBody.put("input_block_y", blockY);
        answerBody.put("input_block_z", blockZ);
        answerBody.put("stream_decoded_block_x", streamDecodedPos.getX());
        answerBody.put("stream_decoded_block_y", streamDecodedPos.getY());
        answerBody.put("stream_decoded_block_z", streamDecodedPos.getZ());
        answerBody.put("decoded_block_x", decodedPos.getX());
        answerBody.put("decoded_block_y", decodedPos.getY());
        answerBody.put("decoded_block_z", decodedPos.getZ());
        answerBody.put("input_progress", progress);
        answerBody.put("stream_decoded_progress", streamDecoded.getProgress());
        answerBody.put("decoded_progress", decodedBlockDestruction.getProgress());
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
