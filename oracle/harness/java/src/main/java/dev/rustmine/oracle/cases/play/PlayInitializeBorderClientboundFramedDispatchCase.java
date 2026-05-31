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
import net.minecraft.network.protocol.game.ClientboundInitializeBorderPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.level.border.WorldBorder;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlayInitializeBorderClientboundFramedDispatchCase {
    private PlayInitializeBorderClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        double newCenterX = inputFields.get("new_center_x").getAsDouble();
        double newCenterZ = inputFields.get("new_center_z").getAsDouble();
        double oldSize = inputFields.get("old_size").getAsDouble();
        double newSize = inputFields.get("new_size").getAsDouble();
        long lerpTime = inputFields.get("lerp_time").getAsLong();
        int newAbsoluteMaxSize = inputFields.get("new_absolute_max_size").getAsInt();
        int warningBlocks = inputFields.get("warning_blocks").getAsInt();
        int warningTime = inputFields.get("warning_time").getAsInt();

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        fixtureBodyOut.writeDouble(newCenterX);
        fixtureBodyOut.writeDouble(newCenterZ);
        fixtureBodyOut.writeDouble(oldSize);
        fixtureBodyOut.writeDouble(newSize);
        fixtureBodyOut.writeVarLong(lerpTime);
        fixtureBodyOut.writeVarInt(newAbsoluteMaxSize);
        fixtureBodyOut.writeVarInt(warningBlocks);
        fixtureBodyOut.writeVarInt(warningTime);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundInitializeBorderPacket packet =
            ClientboundInitializeBorderPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:initialize_border");

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
        if (!(decodedPacket instanceof ClientboundInitializeBorderPacket decodedInitializeBorder)) {
            throw new IllegalStateException(
                "decoded Play initialize_border as unexpected packet "
                    + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundInitializeBorderPacket.STREAM_CODEC, private ClientboundInitializeBorderPacket(FriendlyByteBuf), private write(FriendlyByteBuf), FriendlyByteBuf.readDouble/writeDouble, readVarLong/writeVarLong, readVarInt/writeVarInt, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundInitializeBorderPacket), ClientGamePacketListener.handleInitializeBorder(ClientboundInitializeBorderPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundInitializeBorderPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:initialize_border",
            decodedPacket,
            "official ClientboundInitializeBorderPacket STREAM_CODEC decode fixture from primitive border fields; no initialized WorldBorder, Level, or game state",
            "newCenterX double, newCenterZ double, oldSize double, newSize double, lerpTime VarLong, newAbsoluteMaxSize VarInt, warningBlocks VarInt, warningTime VarInt",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        putInitializeBorderFields(answerBody, "input", newCenterX, newCenterZ, oldSize, newSize, lerpTime, newAbsoluteMaxSize, warningBlocks, warningTime);
        putInitializeBorderFields(answerBody, "decoded", decodedInitializeBorder);
        answer.put("answer", answerBody);
        return answer;
    }
}
