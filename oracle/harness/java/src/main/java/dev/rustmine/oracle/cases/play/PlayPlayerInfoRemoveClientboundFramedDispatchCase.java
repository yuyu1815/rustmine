package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import com.mojang.authlib.GameProfile;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundPlayerInfoRemovePacket;
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


public final class PlayPlayerInfoRemoveClientboundFramedDispatchCase {
    private PlayPlayerInfoRemoveClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        List<UUID> profileIds = jsonUuidList(inputFields, "profile_ids");
        ClientboundPlayerInfoRemovePacket packet = new ClientboundPlayerInfoRemovePacket(profileIds);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundPlayerInfoRemovePacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundPlayerInfoRemovePacket streamDecoded =
            ClientboundPlayerInfoRemovePacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:player_info_remove");

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
        if (!(decodedPacket instanceof ClientboundPlayerInfoRemovePacket decodedPlayerInfoRemove)) {
            throw new IllegalStateException(
                "decoded Play player_info_remove as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundPlayerInfoRemovePacket(List<UUID>); ClientboundPlayerInfoRemovePacket.STREAM_CODEC; UUIDUtil.STREAM_CODEC; FriendlyByteBuf.readList/writeCollection; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundPlayerInfoRemovePacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundPlayerInfoRemovePacket net.minecraft.core.UUIDUtil net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:player_info_remove",
            decodedPacket,
            "official ClientboundPlayerInfoRemovePacket UUID-list fixture; no GameProfile, session, or initialized player-list state is required",
            "VarInt-prefixed UUID list through UUIDUtil.STREAM_CODEC via ClientboundPlayerInfoRemovePacket.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_profile_ids", uuidStrings(profileIds));
        answerBody.put("stream_decoded_profile_ids", uuidStrings(streamDecoded.profileIds()));
        answerBody.put("decoded_profile_ids", uuidStrings(decodedPlayerInfoRemove.profileIds()));
        answerBody.put("input_profile_id_count", profileIds.size());
        answerBody.put("decoded_profile_id_count", decodedPlayerInfoRemove.profileIds().size());
        answer.put("answer", answerBody);
        return answer;
    }
}
