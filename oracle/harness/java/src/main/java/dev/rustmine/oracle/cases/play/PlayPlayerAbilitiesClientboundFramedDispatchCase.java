package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundPlayerAbilitiesPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.entity.player.Abilities;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlayPlayerAbilitiesClientboundFramedDispatchCase {
    private PlayPlayerAbilitiesClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        Abilities abilities = new Abilities();
        abilities.invulnerable = inputFields.get("invulnerable").getAsBoolean();
        abilities.flying = inputFields.get("flying").getAsBoolean();
        abilities.mayfly = inputFields.get("can_fly").getAsBoolean();
        abilities.instabuild = inputFields.get("instabuild").getAsBoolean();
        abilities.setFlyingSpeed(inputFields.get("flying_speed").getAsFloat());
        abilities.setWalkingSpeed(inputFields.get("walking_speed").getAsFloat());
        ClientboundPlayerAbilitiesPacket packet = new ClientboundPlayerAbilitiesPacket(abilities);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundPlayerAbilitiesPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundPlayerAbilitiesPacket streamDecoded =
            ClientboundPlayerAbilitiesPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:player_abilities");

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
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
        if (!(decodedPacket instanceof ClientboundPlayerAbilitiesPacket decodedAbilities)) {
            throw new IllegalStateException(
                "decoded Play player_abilities as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "Abilities public fields and speed accessors; ClientboundPlayerAbilitiesPacket(Abilities); ClientboundPlayerAbilitiesPacket.STREAM_CODEC; FriendlyByteBuf.readByte/writeByte; FriendlyByteBuf.readFloat/writeFloat; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundPlayerAbilitiesPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundPlayerAbilitiesPacket net.minecraft.world.entity.player.Abilities net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:player_abilities",
            decodedPacket,
            "official ClientboundPlayerAbilitiesPacket Abilities fixture using booleans and speed floats only; no initialized player object is required",
            "flags byte bits invulnerable=1, flying=2, canFly=4, instabuild=8, followed by flyingSpeed float and walkingSpeed float through ClientboundPlayerAbilitiesPacket.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        putPlayerAbilitiesFields(answerBody, "input", packet);
        putPlayerAbilitiesFields(answerBody, "stream_decoded", streamDecoded);
        putPlayerAbilitiesFields(answerBody, "decoded", decodedAbilities);
        answer.put("answer", answerBody);
        return answer;
    }
}
