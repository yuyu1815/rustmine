package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundUpdateMobEffectPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.effect.MobEffectInstance;
import net.minecraft.world.effect.MobEffects;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;


public final class PlayUpdateMobEffectClientboundFramedDispatchCase {
    private PlayUpdateMobEffectClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        int entityId = 12345;
        int duration = 200;
        int amplifier = 1;
        boolean ambient = false;
        boolean visible = true;
        boolean showIcon = true;
        boolean blend = false;
        MobEffectInstance effect = new MobEffectInstance(
            MobEffects.SPEED,
            duration,
            amplifier,
            ambient,
            visible,
            showIcon
        );
        ClientboundUpdateMobEffectPacket packet =
            new ClientboundUpdateMobEffectPacket(entityId, effect, blend);

        RegistryAccess registryAccess = RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY);
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundUpdateMobEffectPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundUpdateMobEffectPacket streamDecoded =
            ClientboundUpdateMobEffectPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:update_mob_effect");

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
        if (!(decodedPacket instanceof ClientboundUpdateMobEffectPacket decodedEffect)) {
            throw new IllegalStateException(
                "decoded Play update_mob_effect as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundUpdateMobEffectPacket(int, MobEffectInstance, boolean); MobEffects.SPEED; MobEffect.STREAM_CODEC; ByteBufCodecs.holderRegistry; MobEffectInstance constructor; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.fromRegistryOfRegistries(BuiltInRegistries.REGISTRY))).codec().encode/decode(ClientboundUpdateMobEffectPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundUpdateMobEffectPacket net.minecraft.world.effect.MobEffect net.minecraft.world.effect.MobEffectInstance net.minecraft.network.codec.ByteBufCodecs net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:update_mob_effect",
            decodedPacket,
            "official ClientboundUpdateMobEffectPacket(12345, new MobEffectInstance(MobEffects.SPEED, 200, 1, false, true, true), false) fixture",
            "entity id, MobEffect holder registry id, amplifier, duration, and flags",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_entity_id", entityId);
        answerBody.put("stream_decoded_entity_id", streamDecoded.getEntityId());
        answerBody.put("decoded_entity_id", decodedEffect.getEntityId());
        answerBody.put("input_effect_holder", "MobEffects.SPEED");
        answerBody.put("decoded_effect_description_id", decodedEffect.getEffect().value().getDescriptionId());
        answerBody.put("stream_decoded_effect_description_id", streamDecoded.getEffect().value().getDescriptionId());
        answerBody.put("input_amplifier", amplifier);
        answerBody.put("stream_decoded_amplifier", streamDecoded.getEffectAmplifier());
        answerBody.put("decoded_amplifier", decodedEffect.getEffectAmplifier());
        answerBody.put("input_effect_duration", duration);
        answerBody.put("stream_decoded_effect_duration", streamDecoded.getEffectDurationTicks());
        answerBody.put("decoded_effect_duration", decodedEffect.getEffectDurationTicks());
        answerBody.put("input_ambient", ambient);
        answerBody.put("stream_decoded_ambient", streamDecoded.isEffectAmbient());
        answerBody.put("decoded_ambient", decodedEffect.isEffectAmbient());
        answerBody.put("input_visible", visible);
        answerBody.put("stream_decoded_visible", streamDecoded.isEffectVisible());
        answerBody.put("decoded_visible", decodedEffect.isEffectVisible());
        answerBody.put("input_show_icon", showIcon);
        answerBody.put("stream_decoded_show_icon", streamDecoded.effectShowsIcon());
        answerBody.put("decoded_show_icon", decodedEffect.effectShowsIcon());
        answerBody.put("input_blend", blend);
        answerBody.put("stream_decoded_blend", streamDecoded.shouldBlend());
        answerBody.put("decoded_blend", decodedEffect.shouldBlend());
        answer.put("answer", answerBody);
        return answer;
    }
}
