package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import com.mojang.brigadier.context.StringRange;
import com.mojang.brigadier.suggestion.Suggestions;
import io.netty.buffer.Unpooled;
import java.time.Instant;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientGamePacketListener;
import net.minecraft.network.protocol.game.ClientboundCommandSuggestionsPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.network.chat.Component;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class PlayCommandSuggestionsClientboundFramedDispatchCase {
    private PlayCommandSuggestionsClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        JsonObject inputFields = input.getAsJsonObject("question").getAsJsonObject("input_fields");
        int commandId = inputFields.get("command_id").getAsInt();
        int rangeStart = inputFields.get("range_start").getAsInt();
        int rangeEnd = inputFields.get("range_end").getAsInt();
        Suggestions suggestions = new Suggestions(StringRange.between(rangeStart, rangeEnd), List.of());
        ClientboundCommandSuggestionsPacket packet =
            new ClientboundCommandSuggestionsPacket(commandId, suggestions);
        RegistryAccess registryAccess = RegistryAccess.EMPTY;

        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundCommandSuggestionsPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundCommandSuggestionsPacket streamDecoded =
            ClientboundCommandSuggestionsPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = new ArrayList<>();
        final int[] commandSuggestionsPacketId = { -1 };
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((packetType, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", packetType.id().toString());
            row.put("flow", packetType.flow().id());
            playClientboundPackets.add(row);
            if ("minecraft:command_suggestions".equals(packetType.id().toString())) {
                commandSuggestionsPacketId[0] = packetId;
            }
        });
        if (commandSuggestionsPacketId[0] < 0) {
            throw new IllegalStateException("missing official Play clientbound command_suggestions packet id");
        }

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
        if (!(decodedPacket instanceof ClientboundCommandSuggestionsPacket decodedCommandSuggestions)) {
            throw new IllegalStateException(
                "decoded Play command_suggestions as unexpected packet "
                    + decodedPacket.getClass().getName()
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
            "function_or_member", "ClientboundCommandSuggestionsPacket(int, Suggestions), ClientboundCommandSuggestionsPacket.STREAM_CODEC, ClientboundCommandSuggestionsPacket(int, int, int, List<Entry>), ClientboundCommandSuggestionsPacket.toSuggestions(), ClientboundCommandSuggestionsPacket.Entry.STREAM_CODEC, ByteBufCodecs.VAR_INT, ByteBufCodecs.STRING_UTF8, ComponentSerialization.TRUSTED_OPTIONAL_STREAM_CODEC, GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(...), ClientGamePacketListener.handleCommandSuggestions(ClientboundCommandSuggestionsPacket)",
            "bytecode_source_command", "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundCommandSuggestionsPacket 'net.minecraft.network.protocol.game.ClientboundCommandSuggestionsPacket$Entry' com.mojang.brigadier.suggestion.Suggestions com.mojang.brigadier.context.StringRange net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes net.minecraft.network.protocol.game.ClientGamePacketListener"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Play");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:command_suggestions");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("fixture", "official ClientboundCommandSuggestionsPacket(int, Suggestions) constructor fixture with command id, StringRange.between(rangeStart, rangeEnd), and an empty suggestion list; context-free Brigadier suggestions body with no command tree, command context, Level, or game state");
        answerBody.put("official_body_shape", "command id VarInt, range start VarInt, range length VarInt, then a VarInt suggestion count followed by Entry records; each Entry is text STRING_UTF8 plus optional trusted Component tooltip, and this fixture uses zero entries");
        answerBody.put("input_command_id", commandId);
        answerBody.put("stream_decoded_command_id", streamDecoded.id());
        answerBody.put("decoded_command_id", decodedCommandSuggestions.id());
        answerBody.put("input_range_start", packet.start());
        answerBody.put("stream_decoded_range_start", streamDecoded.start());
        answerBody.put("decoded_range_start", decodedCommandSuggestions.start());
        answerBody.put("input_range_length", packet.length());
        answerBody.put("stream_decoded_range_length", streamDecoded.length());
        answerBody.put("decoded_range_length", decodedCommandSuggestions.length());
        answerBody.put("input_suggestion_count", packet.suggestions().size());
        answerBody.put("stream_decoded_suggestion_count", streamDecoded.suggestions().size());
        answerBody.put("decoded_suggestion_count", decodedCommandSuggestions.suggestions().size());
        answerBody.put("decoded_to_suggestions_range_start", decodedCommandSuggestions.toSuggestions().getRange().getStart());
        answerBody.put("decoded_to_suggestions_range_length", decodedCommandSuggestions.toSuggestions().getRange().getLength());
        answerBody.put("decoded_to_suggestions_count", decodedCommandSuggestions.toSuggestions().getList().size());
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
