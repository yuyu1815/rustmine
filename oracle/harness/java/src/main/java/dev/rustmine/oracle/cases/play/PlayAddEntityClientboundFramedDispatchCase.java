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
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundAddEntityPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.entity.EntityType;
import net.minecraft.world.phys.Vec3;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlayAddEntityClientboundFramedDispatchCase {
    private PlayAddEntityClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int entityId = inputFields.get("entity_id").getAsInt();
        UUID uuid = UUID.fromString(inputFields.get("uuid").getAsString());
        double x = inputFields.get("x").getAsDouble();
        double y = inputFields.get("y").getAsDouble();
        double z = inputFields.get("z").getAsDouble();
        float xRot = inputFields.get("x_rot_degrees").getAsFloat();
        float yRot = inputFields.get("y_rot_degrees").getAsFloat();
        double yHeadRot = inputFields.get("y_head_rot_degrees").getAsDouble();
        int data = inputFields.get("data").getAsInt();
        Vec3 movement = Vec3.ZERO;

        ClientboundAddEntityPacket packet = new ClientboundAddEntityPacket(
            entityId,
            uuid,
            x,
            y,
            z,
            xRot,
            yRot,
            EntityType.PIG,
            data,
            movement,
            yHeadRot
        );

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] addEntityPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:add_entity".equals(type.id().toString())) {
                addEntityPacketId[0] = packetId;
            }
        });
        if (addEntityPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound add_entity packet id");
        }

        RegistryAccess.Frozen registryAccess =
            RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY);
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
        if (!(decodedPacket instanceof ClientboundAddEntityPacket decodedAddEntity)) {
            throw new IllegalStateException(
                "decoded Play add_entity as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf movementOut = new FriendlyByteBuf(Unpooled.buffer());
        Vec3.LP_STREAM_CODEC.encode(movementOut, movement);
        byte[] movementBody = readableBytes(movementOut);

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
            "function_or_member", "ClientboundAddEntityPacket(int, UUID, double, double, double, float, float, EntityType<?>, int, Vec3, double), ClientboundAddEntityPacket.STREAM_CODEC, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY))).codec().encode/decode(...), EntityType.PIG, BuiltInRegistries.ENTITY_TYPE, Vec3.LP_STREAM_CODEC, ClientGamePacketListener.handleAddEntity(ClientboundAddEntityPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundAddEntityPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener net.minecraft.core.RegistryAccess net.minecraft.core.registries.BuiltInRegistries net.minecraft.world.entity.EntityType net.minecraft.world.phys.Vec3"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:add_entity");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "direct official ClientboundAddEntityPacket constructor with bootstrapped built-in EntityType.PIG and zero Vec3.LP movement");
        answerBody.put("input_entity_id", entityId);
        answerBody.put("decoded_entity_id", decodedAddEntity.getId());
        answerBody.put("input_uuid", uuid.toString());
        answerBody.put("decoded_uuid", decodedAddEntity.getUUID().toString());
        answerBody.put("input_entity_type", "minecraft:pig");
        answerBody.put("decoded_entity_type", BuiltInRegistries.ENTITY_TYPE.getKey(decodedAddEntity.getType()).toString());
        answerBody.put("decoded_entity_type_registry_id", BuiltInRegistries.ENTITY_TYPE.getId(decodedAddEntity.getType()));
        answerBody.put("input_x", x);
        answerBody.put("decoded_x", decodedAddEntity.getX());
        answerBody.put("input_y", y);
        answerBody.put("decoded_y", decodedAddEntity.getY());
        answerBody.put("input_z", z);
        answerBody.put("decoded_z", decodedAddEntity.getZ());
        answerBody.put("input_movement_x", movement.x());
        answerBody.put("decoded_movement_x", decodedAddEntity.getMovement().x());
        answerBody.put("input_movement_y", movement.y());
        answerBody.put("decoded_movement_y", decodedAddEntity.getMovement().y());
        answerBody.put("input_movement_z", movement.z());
        answerBody.put("decoded_movement_z", decodedAddEntity.getMovement().z());
        answerBody.put("encoded_movement_lp_hex", HexFormat.of().formatHex(movementBody));
        answerBody.put("input_x_rot_degrees", xRot);
        answerBody.put("decoded_x_rot_degrees", decodedAddEntity.getXRot());
        answerBody.put("decoded_x_rot_byte", privateByte(decodedAddEntity, "xRot"));
        answerBody.put("input_y_rot_degrees", yRot);
        answerBody.put("decoded_y_rot_degrees", decodedAddEntity.getYRot());
        answerBody.put("decoded_y_rot_byte", privateByte(decodedAddEntity, "yRot"));
        answerBody.put("input_y_head_rot_degrees", yHeadRot);
        answerBody.put("decoded_y_head_rot_degrees", decodedAddEntity.getYHeadRot());
        answerBody.put("decoded_y_head_rot_byte", privateByte(decodedAddEntity, "yHeadRot"));
        answerBody.put("input_data", data);
        answerBody.put("decoded_data", decodedAddEntity.getData());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("play_clientbound_packet_table", playClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }
}
