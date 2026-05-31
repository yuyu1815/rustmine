package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.time.Instant;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundBossEventPacket;
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


public final class PlayBossEventClientboundFramedDispatchCase {
    private PlayBossEventClientboundFramedDispatchCase() {
    }

    private static final class BossEventDispatchCapture {
        private final String operation;
        private final UUID uuid;

        private BossEventDispatchCapture(String operation, UUID uuid) {
            this.operation = operation;
            this.uuid = uuid;
        }

        private static BossEventDispatchCapture capture(ClientboundBossEventPacket packet) {
            final BossEventDispatchCapture[] capture = { null };
            packet.dispatch(new ClientboundBossEventPacket.Handler() {
                @Override
                public void remove(UUID id) {
                    capture[0] = new BossEventDispatchCapture("REMOVE", id);
                }
            });
            if (capture[0] == null) {
                throw new IllegalStateException("expected boss_event REMOVE dispatch");
            }
            return capture[0];
        }
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        UUID bossEventId = UUID.fromString(inputFields.get("uuid").getAsString());
        String operation = inputFields.get("operation").getAsString();
        if (!"REMOVE".equals(operation)) {
            throw new IllegalArgumentException("minimal boss_event fixture is scoped to the REMOVE operation");
        }

        ClientboundBossEventPacket packet = ClientboundBossEventPacket.createRemovePacket(bossEventId);
        RegistryAccess registryAccess = RegistryAccess.EMPTY;

        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundBossEventPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundBossEventPacket streamDecoded =
            ClientboundBossEventPacket.STREAM_CODEC.decode(packetIn);
        BossEventDispatchCapture streamCapture = BossEventDispatchCapture.capture(streamDecoded);

        RegistryFriendlyByteBuf operationReader =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        UUID bodyUuid = operationReader.readUUID();
        int operationOrdinal = operationReader.readVarInt();

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] bossEventPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((packetType, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", packetType.id().toString());
            row.put("flow", packetType.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:boss_event".equals(packetType.id().toString())) {
                bossEventPacketId[0] = packetId;
            }
        });
        if (bossEventPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound boss_event packet id");
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
        if (!(decodedPacket instanceof ClientboundBossEventPacket decodedBossEvent)) {
            throw new IllegalStateException(
                "decoded Play boss_event as unexpected packet " + decodedPacket.getClass().getName()
            );
        }
        BossEventDispatchCapture decodedCapture = BossEventDispatchCapture.capture(decodedBossEvent);

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
            "function_or_member", "ClientboundBossEventPacket.createRemovePacket(UUID), ClientboundBossEventPacket.STREAM_CODEC, private write(RegistryFriendlyByteBuf), private ClientboundBossEventPacket(RegistryFriendlyByteBuf), RegistryFriendlyByteBuf.writeUUID/readUUID, RegistryFriendlyByteBuf.writeEnum/readEnum, REMOVE_OPERATION.write(...), ClientboundBossEventPacket.dispatch(Handler), GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...), ClientGamePacketListener.handleBossUpdate(ClientboundBossEventPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundBossEventPacket 'net.minecraft.network.protocol.game.ClientboundBossEventPacket$OperationType' 'net.minecraft.network.protocol.game.ClientboundBossEventPacket$1' net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:boss_event");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundBossEventPacket.createRemovePacket(UUID) fixture; context-free remove operation with no initialized BossEvent, Level, or game state");
        answerBody.put("official_body_shape", "UUID encoded with RegistryFriendlyByteBuf.writeUUID, operation encoded with RegistryFriendlyByteBuf.writeEnum/readEnum, and REMOVE operation writes no additional body");
        answerBody.put("input_uuid", bossEventId.toString());
        answerBody.put("body_uuid", bodyUuid.toString());
        answerBody.put("stream_decoded_uuid", streamCapture.uuid.toString());
        answerBody.put("decoded_uuid", decodedCapture.uuid.toString());
        answerBody.put("input_operation", operation);
        answerBody.put("stream_decoded_operation", streamCapture.operation);
        answerBody.put("decoded_operation", decodedCapture.operation);
        answerBody.put("decoded_operation_ordinal", operationOrdinal);
        answerBody.put("remaining_after_operation_read", operationReader.readableBytes());
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
