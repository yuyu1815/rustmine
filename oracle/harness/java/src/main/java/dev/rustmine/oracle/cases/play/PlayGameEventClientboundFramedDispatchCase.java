package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundGameEventPacket;
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


public final class PlayGameEventClientboundFramedDispatchCase {
    private PlayGameEventClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        String eventName = inputFields.get("event").getAsString();
        ClientboundGameEventPacket.Type event = gameEventType(eventName);
        float param = inputFields.get("param").getAsFloat();
        ClientboundGameEventPacket packet = new ClientboundGameEventPacket(event, param);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundGameEventPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundGameEventPacket streamDecoded =
            ClientboundGameEventPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:game_event");

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
        Packet<? super ClientGamePacketListener> decodedPacket =
            protocolInfo.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundGameEventPacket decodedGameEvent)) {
            throw new IllegalStateException(
                "decoded Play game_event as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundGameEventPacket(Type, float), ClientboundGameEventPacket.STREAM_CODEC, private ClientboundGameEventPacket(FriendlyByteBuf), private write(FriendlyByteBuf), FriendlyByteBuf.readUnsignedByte/writeByte, FriendlyByteBuf.readFloat/writeFloat, ClientboundGameEventPacket.Type id table, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundGameEventPacket), ClientboundGameEventPacket.getEvent(), getParam()",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p -verbose net.minecraft.network.protocol.game.ClientboundGameEventPacket 'net.minecraft.network.protocol.game.ClientboundGameEventPacket$Type' net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:game_event",
            decodedPacket,
            "official ClientboundGameEventPacket Type/float constructor fixture; no initialized Level, player, weather, game mode, or game state",
            "event Type id encoded as one unsigned byte followed by param as one float through ClientboundGameEventPacket.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_game_event", eventName);
        answerBody.put("stream_decoded_game_event", gameEventName(streamDecoded.getEvent()));
        answerBody.put("decoded_game_event", gameEventName(decodedGameEvent.getEvent()));
        answerBody.put("input_game_event_id", privateInt(event, "id"));
        answerBody.put("stream_decoded_game_event_id", privateInt(streamDecoded.getEvent(), "id"));
        answerBody.put("decoded_game_event_id", privateInt(decodedGameEvent.getEvent(), "id"));
        answerBody.put("input_param", param);
        answerBody.put("stream_decoded_param", streamDecoded.getParam());
        answerBody.put("decoded_param", decodedGameEvent.getParam());
        answer.put("answer", answerBody);
        return answer;
    }
}
