package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundSetPlayerTeamPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.scores.PlayerTeam;
import net.minecraft.world.scores.Scoreboard;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;

public final class PlaySetPlayerTeamClientboundFramedDispatchCase {
    private PlaySetPlayerTeamClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String teamName = inputFields.get("team_name").getAsString();
        PlayerTeam team = new PlayerTeam(new Scoreboard(), teamName);
        ClientboundSetPlayerTeamPacket packet = ClientboundSetPlayerTeamPacket.createRemovePacket(team);

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundSetPlayerTeamPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundSetPlayerTeamPacket streamDecoded =
            ClientboundSetPlayerTeamPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:set_player_team");

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
        if (!(decodedPacket instanceof ClientboundSetPlayerTeamPacket decodedTeam)) {
            throw new IllegalStateException(
                "decoded Play set_player_team as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "PlayerTeam(Scoreboard, String), ClientboundSetPlayerTeamPacket.createRemovePacket(PlayerTeam), ClientboundSetPlayerTeamPacket.write/STREAM_CODEC remove branch writes only team name and method byte, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundSetPlayerTeamPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundSetPlayerTeamPacket net.minecraft.network.protocol.game.ClientboundSetPlayerTeamPacket\\$Parameters net.minecraft.world.scores.PlayerTeam net.minecraft.world.scores.Scoreboard net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:set_player_team",
            decodedPacket,
            "official ClientboundSetPlayerTeamPacket.createRemovePacket(PlayerTeam) fixture; remove branch serializes only team name and method byte",
            "team name String followed by signed method byte 1; parameters and player list are absent on the remove branch",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_team_name", teamName);
        answerBody.put("stream_decoded_team_name", streamDecoded.getName());
        answerBody.put("decoded_team_name", decodedTeam.getName());
        answerBody.put("input_method", 1);
        answerBody.put("stream_decoded_method", 1);
        answerBody.put("decoded_method", 1);
        answerBody.put("input_team_parameters_present", packet.getParameters().isPresent());
        answerBody.put("stream_decoded_team_parameters_present", streamDecoded.getParameters().isPresent());
        answerBody.put("decoded_team_parameters_present", decodedTeam.getParameters().isPresent());
        answerBody.put("input_player_count", packet.getPlayers().size());
        answerBody.put("stream_decoded_player_count", streamDecoded.getPlayers().size());
        answerBody.put("decoded_player_count", decodedTeam.getPlayers().size());
        answer.put("answer", answerBody);
        return answer;
    }
}
