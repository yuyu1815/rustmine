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
import net.minecraft.network.protocol.game.ClientboundMountScreenOpenPacket;
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


public final class PlayMountScreenOpenClientboundFramedDispatchCase {
    private PlayMountScreenOpenClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int containerId = inputFields.get("container_id").getAsInt();
        int inventoryColumns = inputFields.get("inventory_columns").getAsInt();
        int entityId = inputFields.get("entity_id").getAsInt();
        ClientboundMountScreenOpenPacket packet =
            new ClientboundMountScreenOpenPacket(containerId, inventoryColumns, entityId);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundMountScreenOpenPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundMountScreenOpenPacket streamDecoded =
            ClientboundMountScreenOpenPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:mount_screen_open");

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
        if (!(decodedPacket instanceof ClientboundMountScreenOpenPacket decodedMount)) {
            throw new IllegalStateException(
                "decoded Play mount_screen_open as unexpected packet "
                    + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundMountScreenOpenPacket(int, int, int), ClientboundMountScreenOpenPacket.STREAM_CODEC, FriendlyByteBuf.readContainerId/writeContainerId, FriendlyByteBuf.readVarInt/writeVarInt, FriendlyByteBuf.readInt/writeInt, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundMountScreenOpenPacket), ClientboundMountScreenOpenPacket.getContainerId(), getInventoryColumns(), getEntityId()",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundMountScreenOpenPacket net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:mount_screen_open",
            decodedPacket,
            "official ClientboundMountScreenOpenPacket primitive constructor fixture; no initialized mount entity, inventory, screen, Level, or game state",
            "container id encoded by FriendlyByteBuf.writeContainerId, inventory columns by VarInt, and entity id by big-endian int through ClientboundMountScreenOpenPacket.STREAM_CODEC",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_container_id", containerId);
        answerBody.put("stream_decoded_container_id", streamDecoded.getContainerId());
        answerBody.put("decoded_container_id", decodedMount.getContainerId());
        answerBody.put("input_inventory_columns", inventoryColumns);
        answerBody.put("stream_decoded_inventory_columns", streamDecoded.getInventoryColumns());
        answerBody.put("decoded_inventory_columns", decodedMount.getInventoryColumns());
        answerBody.put("input_entity_id", entityId);
        answerBody.put("stream_decoded_entity_id", streamDecoded.getEntityId());
        answerBody.put("decoded_entity_id", decodedMount.getEntityId());
        answer.put("answer", answerBody);
        return answer;
    }
}
