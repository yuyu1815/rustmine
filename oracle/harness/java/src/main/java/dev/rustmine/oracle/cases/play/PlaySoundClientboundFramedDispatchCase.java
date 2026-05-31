package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.Holder;
import net.minecraft.core.RegistryAccess;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundSoundPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.sounds.SoundEvent;
import net.minecraft.sounds.SoundEvents;
import net.minecraft.sounds.SoundSource;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;


public final class PlaySoundClientboundFramedDispatchCase {
    private PlaySoundClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        Holder<SoundEvent> sound = SoundEvents.AMBIENT_CAVE;
        SoundSource source = SoundSource.MASTER;
        double x = 1.25D;
        double y = 64.5D;
        double z = -2.75D;
        float volume = 0.75F;
        float pitch = 1.25F;
        long seed = 123456789L;
        ClientboundSoundPacket packet = new ClientboundSoundPacket(
            sound,
            source,
            x,
            y,
            z,
            volume,
            pitch,
            seed
        );

        RegistryAccess registryAccess = RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY);
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundSoundPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundSoundPacket streamDecoded =
            ClientboundSoundPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:sound");

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
        Packet<?> decodedPacket = protocolInfo.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundSoundPacket decodedSound)) {
            throw new IllegalStateException(
                "decoded Play sound as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundSoundPacket(Holder<SoundEvent>, SoundSource, double, double, double, float, float, long); SoundEvents.AMBIENT_CAVE; SoundEvent.STREAM_CODEC; ByteBufCodecs.holder direct/reference branch; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY))).codec().encode/decode(ClientboundSoundPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSoundPacket net.minecraft.sounds.SoundEvent net.minecraft.network.codec.ByteBufCodecs net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:sound",
            decodedPacket,
            "official ClientboundSoundPacket(SoundEvents.AMBIENT_CAVE, SoundSource.MASTER, 1.25, 64.5, -2.75, 0.75F, 1.25F, 123456789L) fixture",
            "SoundEvent holder id plus source enum, quantized position ints, volume, pitch, and seed",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_sound_holder", "SoundEvents.AMBIENT_CAVE");
        answerBody.put("decoded_sound_location", decodedSound.getSound().value().location().toString());
        answerBody.put("stream_decoded_sound_location", streamDecoded.getSound().value().location().toString());
        answerBody.put("input_source", source.getName());
        answerBody.put("stream_decoded_source", streamDecoded.getSource().getName());
        answerBody.put("decoded_source", decodedSound.getSource().getName());
        answerBody.put("input_position_x", x);
        answerBody.put("stream_decoded_position_x", streamDecoded.getX());
        answerBody.put("decoded_position_x", decodedSound.getX());
        answerBody.put("input_position_y", y);
        answerBody.put("stream_decoded_position_y", streamDecoded.getY());
        answerBody.put("decoded_position_y", decodedSound.getY());
        answerBody.put("input_position_z", z);
        answerBody.put("stream_decoded_position_z", streamDecoded.getZ());
        answerBody.put("decoded_position_z", decodedSound.getZ());
        answerBody.put("input_volume", volume);
        answerBody.put("stream_decoded_volume", streamDecoded.getVolume());
        answerBody.put("decoded_volume", decodedSound.getVolume());
        answerBody.put("input_pitch", pitch);
        answerBody.put("stream_decoded_pitch", streamDecoded.getPitch());
        answerBody.put("decoded_pitch", decodedSound.getPitch());
        answerBody.put("input_seed", seed);
        answerBody.put("stream_decoded_seed", streamDecoded.getSeed());
        answerBody.put("decoded_seed", decodedSound.getSeed());
        answer.put("answer", answerBody);
        return answer;
    }
}
