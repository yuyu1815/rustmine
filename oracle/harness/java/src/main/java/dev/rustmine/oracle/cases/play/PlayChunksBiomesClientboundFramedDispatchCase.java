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
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundChunksBiomesPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.level.ChunkPos;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlayChunksBiomesClientboundFramedDispatchCase {
    private PlayChunksBiomesClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        ClientboundChunksBiomesPacket packet =
            new ClientboundChunksBiomesPacket(List.of());
        RegistryAccess registryAccess = RegistryAccess.EMPTY;

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundChunksBiomesPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundChunksBiomesPacket streamDecoded =
            ClientboundChunksBiomesPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] chunksBiomesPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((packetType, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", packetType.id().toString());
            row.put("flow", packetType.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:chunks_biomes".equals(packetType.id().toString())) {
                chunksBiomesPacketId[0] = packetId;
            }
        });
        if (chunksBiomesPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound chunks_biomes packet id");
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
        if (!(decodedPacket instanceof ClientboundChunksBiomesPacket decodedChunksBiomes)) {
            throw new IllegalStateException(
                "decoded Play chunks_biomes as unexpected packet "
                    + decodedPacket.getClass().getName()
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
            "function_or_member", "ClientboundChunksBiomesPacket(List<ChunkBiomeData>), ClientboundChunksBiomesPacket.STREAM_CODEC, private ClientboundChunksBiomesPacket(FriendlyByteBuf), private write(FriendlyByteBuf), FriendlyByteBuf.readList/writeCollection, ClientboundChunksBiomesPacket.ChunkBiomeData(ChunkPos, byte[]), ChunkBiomeData(FriendlyByteBuf), ChunkBiomeData.write(FriendlyByteBuf), FriendlyByteBuf.readChunkPos/writeChunkPos, FriendlyByteBuf.readByteArray(2097152)/writeByteArray, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...), ClientGamePacketListener.handleChunksBiomes(ClientboundChunksBiomesPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundChunksBiomesPacket net.minecraft.network.protocol.game.ClientboundChunksBiomesPacket\\$ChunkBiomeData net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:chunks_biomes");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundChunksBiomesPacket(List<ChunkBiomeData>) constructor fixture with an empty chunkBiomeData list; context-free list-count body with no initialized Level, LevelChunk, biome registry, chunk storage, or game state");
        answerBody.put("official_body_shape", "chunkBiomeData encoded as FriendlyByteBuf VarInt list count followed by each ChunkBiomeData as ChunkPos via FriendlyByteBuf.writeChunkPos and biome byte array via FriendlyByteBuf.writeByteArray/readByteArray(max 2097152); empty fixture encodes only list count 0");
        answerBody.put("input_chunk_biome_count", packet.chunkBiomeData().size());
        answerBody.put("stream_decoded_chunk_biome_count", streamDecoded.chunkBiomeData().size());
        answerBody.put("decoded_chunk_biome_count", decodedChunksBiomes.chunkBiomeData().size());
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
