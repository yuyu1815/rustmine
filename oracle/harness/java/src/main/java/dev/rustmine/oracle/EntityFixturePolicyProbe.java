package dev.rustmine.oracle;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HexFormat;
import java.util.function.BiConsumer;
import java.util.function.Consumer;
import net.minecraft.core.RegistryAccess;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.core.registries.Registries;
import net.minecraft.SharedConstants;
import net.minecraft.gametest.framework.GameTestHelper;
import net.minecraft.gametest.framework.GameTestMainUtil;
import net.minecraft.gametest.framework.TestFunctionLoader;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundSetEntityLinkPacket;
import net.minecraft.network.protocol.game.ClientboundSetPassengersPacket;
import net.minecraft.network.protocol.game.ClientboundSoundEntityPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.resources.Identifier;
import net.minecraft.resources.ResourceKey;
import net.minecraft.sounds.SoundEvents;
import net.minecraft.sounds.SoundSource;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntityType;
import net.minecraft.world.phys.Vec3;

public final class EntityFixturePolicyProbe {
    private static final Identifier TEST_ID = Identifier.parse("rustmine:entity_packet_probe");
    private static final ResourceKey<Consumer<GameTestHelper>> TEST_FUNCTION =
        ResourceKey.create(Registries.TEST_FUNCTION, TEST_ID);

    private EntityFixturePolicyProbe() {
    }

    public static void main(String[] args) throws Exception {
        if (args.length != 1) {
            throw new IllegalArgumentException(
                "usage: EntityFixturePolicyProbe <output-jsonl-path>"
            );
        }
        Path output = Path.of(args[0]).toAbsolutePath();
        Files.createDirectories(output.getParent());
        Files.deleteIfExists(output);
        SharedConstants.tryDetectVersion();

        TestFunctionLoader.registerLoader(new TestFunctionLoader() {
            @Override
            public void load(
                BiConsumer<ResourceKey<Consumer<GameTestHelper>>, Consumer<GameTestHelper>> consumer
            ) {
                consumer.accept(TEST_FUNCTION, helper -> runProbe(helper, output));
            }
        });

        Path work = Files.createTempDirectory("rustmine-entity-fixture-policy-");
        Path packs = work.resolve("packs");
        writeProbePack(packs.resolve("rustmine_entity_fixture_policy"));
        GameTestMainUtil.runGameTestServer(
            new String[] {
                "--universe", work.resolve("universe").toString(),
                "--packs", packs.toString(),
                "--tests", TEST_ID.toString()
            },
            ignored -> { }
        );
    }

    private static void runProbe(GameTestHelper helper, Path output) {
        try {
            Entity source = helper.spawn(EntityType.PIG, new Vec3(1.0D, 2.0D, 1.0D));
            Entity destination = helper.spawn(EntityType.ARMOR_STAND, new Vec3(2.0D, 2.0D, 1.0D));
            Entity vehicle = helper.spawn(EntityType.MINECART, new Vec3(3.0D, 2.0D, 1.0D));
            Entity passenger = helper.spawn(EntityType.PIG, new Vec3(3.0D, 2.0D, 1.0D));
            if (!passenger.startRiding(vehicle)) {
                throw new IllegalStateException("official passenger.startRiding(vehicle) returned false");
            }

            RegistryAccess registryAccess = RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY);
            JsonObject row = new JsonObject();
            row.addProperty("source_entity_class", source.getClass().getName());
            row.addProperty("source_entity_id", source.getId());
            row.addProperty("destination_entity_class", destination.getClass().getName());
            row.addProperty("destination_entity_id", destination.getId());
            row.addProperty("vehicle_entity_class", vehicle.getClass().getName());
            row.addProperty("vehicle_entity_id", vehicle.getId());
            row.addProperty("passenger_entity_class", passenger.getClass().getName());
            row.addProperty("passenger_entity_id", passenger.getId());
            row.addProperty("vehicle_passenger_count", vehicle.getPassengers().size());

            addPacket(row, registryAccess, "set_entity_link",
                new ClientboundSetEntityLinkPacket(source, destination));
            addPacket(row, registryAccess, "set_passengers",
                new ClientboundSetPassengersPacket(vehicle));
            addPacket(row, registryAccess, "sound_entity",
                new ClientboundSoundEntityPacket(
                    SoundEvents.AMBIENT_CAVE,
                    SoundSource.MASTER,
                    source,
                    0.75F,
                    1.25F,
                    123456789L
                ));

            Files.writeString(output, row + "\n", StandardCharsets.UTF_8);
            helper.succeed();
        } catch (Exception error) {
            throw new RuntimeException(error);
        }
    }

    private static void addPacket(
        JsonObject row,
        RegistryAccess registryAccess,
        String name,
        Packet<? super ClientGamePacketListener> packet
    ) {
        var protocolInfo = GameProtocols.CLIENTBOUND_TEMPLATE.bind(
            RegistryFriendlyByteBuf.decorator(registryAccess)
        );
        RegistryFriendlyByteBuf framedOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        protocolInfo.codec().encode(framedOut, packet);
        byte[] framed = OracleBuffers.readableBytes(framedOut);
        row.addProperty(name + "_decoded_packet_type", packet.type().id().toString());
        row.addProperty(name + "_encoded_framed_hex", HexFormat.of().formatHex(framed));
        row.addProperty(
            name + "_encoded_body_hex",
            HexFormat.of().formatHex(OracleBuffers.bytesAfterVarIntPrefix(framed))
        );
    }

    private static void writeProbePack(Path pack) throws Exception {
        Files.createDirectories(pack.resolve("data/rustmine/test_instance"));
        Files.writeString(
            pack.resolve("pack.mcmeta"),
            """
            {
              "pack": {
                "pack_format": 80,
                "description": "Rustmine entity fixture policy probe"
              }
            }
            """,
            StandardCharsets.UTF_8
        );
        Files.writeString(
            pack.resolve("data/rustmine/test_instance/entity_packet_probe.json"),
            """
            {
              "type": "minecraft:function",
              "function": "rustmine:entity_packet_probe",
              "environment": "minecraft:default",
              "structure": "minecraft:empty",
              "max_ticks": 20,
              "required": true
            }
            """,
            StandardCharsets.UTF_8
        );
    }
}
