package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import com.mojang.brigadier.CommandDispatcher;
import com.mojang.brigadier.tree.ArgumentCommandNode;
import com.mojang.brigadier.tree.CommandNode;
import com.mojang.brigadier.tree.RootCommandNode;
import io.netty.buffer.Unpooled;
import java.time.Instant;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundCommandsPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.resources.Identifier;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlayCommandsClientboundFramedDispatchCase {
    private PlayCommandsClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        CommandDispatcher<Object> dispatcher = new CommandDispatcher<>();
        RootCommandNode<Object> root = dispatcher.getRoot();
        ClientboundCommandsPacket.NodeInspector<Object> inspector =
            new ClientboundCommandsPacket.NodeInspector<>() {
                @Override
                public Identifier suggestionId(ArgumentCommandNode<Object, ?> node) {
                    return null;
                }

                @Override
                public boolean isExecutable(CommandNode<Object> node) {
                    return node.getCommand() != null;
                }

                @Override
                public boolean isRestricted(CommandNode<Object> node) {
                    return false;
                }
            };
        ClientboundCommandsPacket packet = new ClientboundCommandsPacket(root, inspector);

        FriendlyByteBuf fixtureBodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundCommandsPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        FriendlyByteBuf packetIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody));
        ClientboundCommandsPacket streamDecoded =
            ClientboundCommandsPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] commandsPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((packetType, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", packetType.id().toString());
            row.put("flow", packetType.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:commands".equals(packetType.id().toString())) {
                commandsPacketId[0] = packetId;
            }
        });
        if (commandsPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound commands packet id");
        }

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
        if (!(decodedPacket instanceof ClientboundCommandsPacket decodedCommands)) {
            throw new IllegalStateException(
                "decoded Play commands as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = new LinkedHashMap<>();
        answer.put("case_id", input.get("case_id").getAsString());
        answer.put("generated_by", Map.of(
            "tool", "oracle/harness/java",
            "version_manifest", "oracle/versions/26.1.2.toml",
            "timestamp_utc", Instant.now().toString()
        ));
        answer.put("official_source", Map.of(
            "jar_role", "client",
            "jar_path", "_analysis/minecraft-26.1.2/client.jar",
            "sha1", "4e618f09a0c649dde3fdf829df443ce0b8831e65",
            "function_or_member", "ClientboundCommandsPacket(RootCommandNode<S>, NodeInspector<S>), ClientboundCommandsPacket.STREAM_CODEC, ClientboundCommandsPacket(FriendlyByteBuf), ClientboundCommandsPacket.write(FriendlyByteBuf), Entry.write(FriendlyByteBuf), GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...), ClientGamePacketListener.handleCommands(ClientboundCommandsPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundCommandsPacket 'net.minecraft.network.protocol.game.ClientboundCommandsPacket$Entry' 'net.minecraft.network.protocol.game.ClientboundCommandsPacket$NodeInspector' 'net.minecraft.network.protocol.game.ClientboundCommandsPacket$NodeStub' com.mojang.brigadier.CommandDispatcher com.mojang.brigadier.tree.RootCommandNode net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:commands");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundCommandsPacket(RootCommandNode<S>, NodeInspector<S>) constructor fixture with an empty Brigadier CommandDispatcher root; context-free root-only command tree with no argument nodes, command context, Level, or game state");
        answerBody.put("official_body_shape", "VarInt node count, then each Entry as flags byte, VarInt child index array, optional redirect index, and node-specific payload, followed by root index VarInt; this root-only fixture has one root Entry with flags 0, zero children, no redirect, no stub payload, and root index 0");
        answerBody.put("input_root_child_count", root.getChildren().size());
        answerBody.put("stream_decoded_entry_count", privateListSize(streamDecoded, "entries"));
        answerBody.put("decoded_entry_count", privateListSize(decodedCommands, "entries"));
        answerBody.put("stream_decoded_root_index", privateInt(streamDecoded, "rootIndex"));
        answerBody.put("decoded_root_index", privateInt(decodedCommands, "rootIndex"));
        answerBody.put("remaining_after_packet_stream_decode", packetIn.readableBytes());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("fixture_body_hex", HexFormat.of().formatHex(fixtureBody));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("play_clientbound_packet_table", playClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }
}
