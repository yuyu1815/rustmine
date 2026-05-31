package dev.rustmine.oracle;

import com.google.gson.Gson;
import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HexFormat;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundSetEntityLinkPacket;
import net.minecraft.network.protocol.game.ClientboundSetPassengersPacket;
import net.minecraft.network.protocol.game.ClientboundSoundEntityPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;

public final class EntityFixturePolicyAnswers {
    private static final Gson GSON = new Gson();
    private static JsonObject cachedProbe;

    private EntityFixturePolicyAnswers() {
    }

    public static Map<String, Object> generate(JsonObject input, String packetType) {
        JsonObject probe = probe();
        RegistryAccess registryAccess = RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY);
        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, packetType);

        String key = packetType.substring("minecraft:".length());
        byte[] framed = hexToBytes(requireString(probe, key + "_encoded_framed_hex"));
        byte[] body = hexToBytes(requireString(probe, key + "_encoded_body_hex"));

        Packet<?> streamDecoded = streamDecode(packetType, body, registryAccess);
        Packet<?> protocolDecoded = protocolDecode(framed, registryAccess);
        if (!packetType.equals(protocolDecoded.type().id().toString())) {
            throw new IllegalStateException(
                "decoded " + packetType + " as " + protocolDecoded.type().id()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            officialSource(packetType),
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.gametest.framework.GameTestMainUtil net.minecraft.gametest.framework.GameTestHelper net.minecraft.gametest.framework.TestFunctionLoader net.minecraft.network.protocol.game.ClientboundSetEntityLinkPacket net.minecraft.network.protocol.game.ClientboundSetPassengersPacket net.minecraft.network.protocol.game.ClientboundSoundEntityPacket net.minecraft.network.protocol.game.GameProtocols"
        );
        Map<String, Object> answerBody = playAnswerBody(
            packetType,
            protocolDecoded,
            fixture(packetType),
            bodyShape(packetType),
            packetId,
            remainingAfterStreamDecode(packetType, body, registryAccess),
            framed,
            body,
            body,
            remainingAfterProtocolDecode(framed, registryAccess),
            playClientboundPackets
        );
        answerBody.put("game_test_fixture", "rustmine:entity_packet_probe");
        answerBody.put("source_entity_class", requireString(probe, "source_entity_class"));
        answerBody.put("source_entity_id", requireInt(probe, "source_entity_id"));
        answerBody.put("destination_entity_class", requireString(probe, "destination_entity_class"));
        answerBody.put("destination_entity_id", requireInt(probe, "destination_entity_id"));
        answerBody.put("vehicle_entity_class", requireString(probe, "vehicle_entity_class"));
        answerBody.put("vehicle_entity_id", requireInt(probe, "vehicle_entity_id"));
        answerBody.put("passenger_entity_class", requireString(probe, "passenger_entity_class"));
        answerBody.put("passenger_entity_id", requireInt(probe, "passenger_entity_id"));
        answerBody.put("vehicle_passenger_count", requireInt(probe, "vehicle_passenger_count"));
        addPacketFields(packetType, answerBody, streamDecoded, protocolDecoded);
        answer.put("answer", answerBody);
        return answer;
    }

    private static synchronized JsonObject probe() {
        if (cachedProbe != null) {
            return cachedProbe;
        }
        try {
            Path output = Files.createTempFile("rustmine-entity-fixture-policy-answer-", ".jsonl");
            Path stdout = Files.createTempFile("rustmine-entity-fixture-policy-answer-", ".out");
            Path stderr = Files.createTempFile("rustmine-entity-fixture-policy-answer-", ".err");
            String javaBinary = Path.of(System.getProperty("java.home"), "bin", "java").toString();
            Process process = new ProcessBuilder(
                javaBinary,
                "--sun-misc-unsafe-memory-access=allow",
                "-cp",
                System.getProperty("java.class.path"),
                "dev.rustmine.oracle.EntityFixturePolicyProbe",
                output.toString()
            ).redirectOutput(stdout.toFile()).redirectError(stderr.toFile()).start();
            int status = process.waitFor();
            if (status != 0) {
                throw new IllegalStateException(
                    "entity fixture policy probe exited " + status
                    + "\nstdout:\n" + Files.readString(stdout)
                    + "\nstderr:\n" + Files.readString(stderr)
                );
            }
            cachedProbe = GSON.fromJson(Files.readString(output), JsonObject.class);
            return cachedProbe;
        } catch (Exception error) {
            throw new RuntimeException("failed to run entity fixture policy probe", error);
        }
    }

    private static Packet<?> streamDecode(
        String packetType,
        byte[] body,
        RegistryAccess registryAccess
    ) {
        return switch (packetType) {
            case "minecraft:set_entity_link" -> ClientboundSetEntityLinkPacket.STREAM_CODEC.decode(
                new FriendlyByteBuf(Unpooled.wrappedBuffer(body))
            );
            case "minecraft:set_passengers" -> ClientboundSetPassengersPacket.STREAM_CODEC.decode(
                new FriendlyByteBuf(Unpooled.wrappedBuffer(body))
            );
            case "minecraft:sound_entity" -> ClientboundSoundEntityPacket.STREAM_CODEC.decode(
                new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(body), registryAccess)
            );
            default -> throw new IllegalArgumentException("unsupported entity fixture packet: " + packetType);
        };
    }

    private static int remainingAfterStreamDecode(
        String packetType,
        byte[] body,
        RegistryAccess registryAccess
    ) {
        if ("minecraft:sound_entity".equals(packetType)) {
            RegistryFriendlyByteBuf buf =
                new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(body), registryAccess);
            ClientboundSoundEntityPacket.STREAM_CODEC.decode(buf);
            return buf.readableBytes();
        }
        FriendlyByteBuf buf = new FriendlyByteBuf(Unpooled.wrappedBuffer(body));
        if ("minecraft:set_entity_link".equals(packetType)) {
            ClientboundSetEntityLinkPacket.STREAM_CODEC.decode(buf);
        } else if ("minecraft:set_passengers".equals(packetType)) {
            ClientboundSetPassengersPacket.STREAM_CODEC.decode(buf);
        } else {
            throw new IllegalArgumentException("unsupported entity fixture packet: " + packetType);
        }
        return buf.readableBytes();
    }

    private static Packet<?> protocolDecode(byte[] framed, RegistryAccess registryAccess) {
        RegistryFriendlyByteBuf buf =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(framed), registryAccess);
        return GameProtocols.CLIENTBOUND_TEMPLATE
            .bind(RegistryFriendlyByteBuf.decorator(registryAccess))
            .codec()
            .decode(buf);
    }

    private static int remainingAfterProtocolDecode(byte[] framed, RegistryAccess registryAccess) {
        RegistryFriendlyByteBuf buf =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(framed), registryAccess);
        GameProtocols.CLIENTBOUND_TEMPLATE
            .bind(RegistryFriendlyByteBuf.decorator(registryAccess))
            .codec()
            .decode(buf);
        return buf.readableBytes();
    }

    private static void addPacketFields(
        String packetType,
        Map<String, Object> answerBody,
        Packet<?> streamDecoded,
        Packet<?> protocolDecoded
    ) {
        switch (packetType) {
            case "minecraft:set_entity_link" -> {
                ClientboundSetEntityLinkPacket streamLink =
                    (ClientboundSetEntityLinkPacket) streamDecoded;
                ClientboundSetEntityLinkPacket decodedLink =
                    (ClientboundSetEntityLinkPacket) protocolDecoded;
                answerBody.put("stream_decoded_source_entity_id", streamLink.getSourceId());
                answerBody.put("decoded_source_entity_id", decodedLink.getSourceId());
                answerBody.put("stream_decoded_destination_entity_id", streamLink.getDestId());
                answerBody.put("decoded_destination_entity_id", decodedLink.getDestId());
            }
            case "minecraft:set_passengers" -> {
                ClientboundSetPassengersPacket streamPassengers =
                    (ClientboundSetPassengersPacket) streamDecoded;
                ClientboundSetPassengersPacket decodedPassengers =
                    (ClientboundSetPassengersPacket) protocolDecoded;
                answerBody.put("stream_decoded_vehicle_entity_id", streamPassengers.getVehicle());
                answerBody.put("decoded_vehicle_entity_id", decodedPassengers.getVehicle());
                answerBody.put("stream_decoded_passenger_entity_ids", ints(streamPassengers.getPassengers()));
                answerBody.put("decoded_passenger_entity_ids", ints(decodedPassengers.getPassengers()));
            }
            case "minecraft:sound_entity" -> {
                ClientboundSoundEntityPacket streamSound =
                    (ClientboundSoundEntityPacket) streamDecoded;
                ClientboundSoundEntityPacket decodedSound =
                    (ClientboundSoundEntityPacket) protocolDecoded;
                answerBody.put("input_sound_holder", "SoundEvents.AMBIENT_CAVE");
                answerBody.put("stream_decoded_sound_location", streamSound.getSound().value().location().toString());
                answerBody.put("decoded_sound_location", decodedSound.getSound().value().location().toString());
                answerBody.put("input_source", "master");
                answerBody.put("stream_decoded_source", streamSound.getSource().getName());
                answerBody.put("decoded_source", decodedSound.getSource().getName());
                answerBody.put("stream_decoded_entity_id", streamSound.getId());
                answerBody.put("decoded_entity_id", decodedSound.getId());
                answerBody.put("input_volume", 0.75F);
                answerBody.put("stream_decoded_volume", streamSound.getVolume());
                answerBody.put("decoded_volume", decodedSound.getVolume());
                answerBody.put("input_pitch", 1.25F);
                answerBody.put("stream_decoded_pitch", streamSound.getPitch());
                answerBody.put("decoded_pitch", decodedSound.getPitch());
                answerBody.put("input_seed", 123456789L);
                answerBody.put("stream_decoded_seed", streamSound.getSeed());
                answerBody.put("decoded_seed", decodedSound.getSeed());
            }
            default -> throw new IllegalArgumentException("unsupported entity fixture packet: " + packetType);
        }
    }

    private static List<Integer> ints(int[] values) {
        return java.util.Arrays.stream(values).boxed().toList();
    }

    private static String officialSource(String packetType) {
        return switch (packetType) {
            case "minecraft:set_entity_link" ->
                "GameTestMainUtil.runGameTestServer(...); TestFunctionLoader.registerLoader(...); GameTestHelper.spawn(EntityType.PIG/ARMOR_STAND, Vec3) in ServerLevel; ClientboundSetEntityLinkPacket(Entity, Entity); GameProtocols.CLIENTBOUND_TEMPLATE.bind(...).codec().encode/decode(...)";
            case "minecraft:set_passengers" ->
                "GameTestMainUtil.runGameTestServer(...); TestFunctionLoader.registerLoader(...); GameTestHelper.spawn(EntityType.MINECART/PIG, Vec3) in ServerLevel; Entity.startRiding(Entity); ClientboundSetPassengersPacket(Entity); GameProtocols.CLIENTBOUND_TEMPLATE.bind(...).codec().encode/decode(...)";
            case "minecraft:sound_entity" ->
                "GameTestMainUtil.runGameTestServer(...); TestFunctionLoader.registerLoader(...); GameTestHelper.spawn(EntityType.PIG, Vec3) in ServerLevel; SoundEvents.AMBIENT_CAVE; ClientboundSoundEntityPacket(Holder<SoundEvent>, SoundSource, Entity, float, float, long); GameProtocols.CLIENTBOUND_TEMPLATE.bind(...).codec().encode/decode(...)";
            default -> throw new IllegalArgumentException("unsupported entity fixture packet: " + packetType);
        };
    }

    private static String fixture(String packetType) {
        return switch (packetType) {
            case "minecraft:set_entity_link" ->
                "official GameTest entity fixture: source pig id 1 linked to destination armor stand id 2";
            case "minecraft:set_passengers" ->
                "official GameTest entity fixture: minecart vehicle id 3 with passenger pig id 4 via passenger.startRiding(vehicle)";
            case "minecraft:sound_entity" ->
                "official GameTest entity fixture: SoundEvents.AMBIENT_CAVE master source emitted from source pig id 1";
            default -> throw new IllegalArgumentException("unsupported entity fixture packet: " + packetType);
        };
    }

    private static String bodyShape(String packetType) {
        return switch (packetType) {
            case "minecraft:set_entity_link" -> "source entity id int and destination entity id int";
            case "minecraft:set_passengers" -> "vehicle entity id VarInt and VarIntArray passenger entity ids";
            case "minecraft:sound_entity" -> "SoundEvent holder id, SoundSource enum id, entity id VarInt, volume, pitch, and seed";
            default -> throw new IllegalArgumentException("unsupported entity fixture packet: " + packetType);
        };
    }

    private static String requireString(JsonObject object, String key) {
        if (!object.has(key)) {
            throw new IllegalStateException("entity fixture probe missing " + key);
        }
        return object.get(key).getAsString();
    }

    private static int requireInt(JsonObject object, String key) {
        if (!object.has(key)) {
            throw new IllegalStateException("entity fixture probe missing " + key);
        }
        return object.get(key).getAsInt();
    }
}
