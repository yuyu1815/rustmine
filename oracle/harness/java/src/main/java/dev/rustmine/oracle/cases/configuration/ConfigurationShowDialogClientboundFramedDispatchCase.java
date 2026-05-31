package dev.rustmine.oracle.cases.configuration;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.time.Instant;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.configuration.ClientConfigurationPacketListener;
import net.minecraft.network.protocol.configuration.ConfigurationProtocols;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.common.ClientboundShowDialogPacket;
import net.minecraft.core.Holder;
import net.minecraft.server.dialog.CommonDialogData;
import net.minecraft.server.dialog.Dialog;
import net.minecraft.server.dialog.DialogAction;
import net.minecraft.server.dialog.NoticeDialog;
import static dev.rustmine.oracle.OracleAnswerRows.*;
import static dev.rustmine.oracle.OracleCaseInputs.*;
import static dev.rustmine.oracle.OracleGameEvents.*;
import static dev.rustmine.oracle.OraclePacketConstruction.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;
import static dev.rustmine.oracle.OraclePlayFieldWriters.*;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OracleReflection.*;


public final class ConfigurationShowDialogClientboundFramedDispatchCase {
    private ConfigurationShowDialogClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        CommonDialogData common = new CommonDialogData(
            Component.literal("Oracle notice"),
            Optional.empty(),
            true,
            false,
            DialogAction.CLOSE,
            List.of(),
            List.of()
        );
        Dialog dialog = new NoticeDialog(common, NoticeDialog.DEFAULT_ACTION);
        ClientboundShowDialogPacket packet = new ClientboundShowDialogPacket(Holder.direct(dialog));

        FriendlyByteBuf framedOut = new FriendlyByteBuf(Unpooled.buffer());
        ConfigurationProtocols.CLIENTBOUND.codec().encode(framedOut, packet);
        byte[] framed = readableBytes(framedOut);

        FriendlyByteBuf framedIn = new FriendlyByteBuf(Unpooled.wrappedBuffer(framed));
        Packet<? super ClientConfigurationPacketListener> decodedPacket =
            ConfigurationProtocols.CLIENTBOUND.codec().decode(framedIn);
        if (!(decodedPacket instanceof ClientboundShowDialogPacket decodedShowDialog)) {
            throw new IllegalStateException(
                "expected ClientboundShowDialogPacket, got " + decodedPacket.getClass().getName()
            );
        }

        FriendlyByteBuf bodyOut = new FriendlyByteBuf(Unpooled.buffer());
        ClientboundShowDialogPacket.CONTEXT_FREE_STREAM_CODEC.encode(bodyOut, packet);
        byte[] body = readableBytes(bodyOut);
        Dialog decodedDialog = decodedShowDialog.dialog().value();

        List<Map<String, Object>> configurationClientboundPackets = new ArrayList<>();
        ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((type, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", type.id().toString());
            row.put("flow", type.flow().id());
            configurationClientboundPackets.add(row);
        });

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
            "function_or_member", "ClientboundShowDialogPacket(Holder.direct(NoticeDialog)), ClientboundShowDialogPacket.CONTEXT_FREE_STREAM_CODEC, Dialog.CONTEXT_FREE_STREAM_CODEC, ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundShowDialogPacket), ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...), ClientboundShowDialogPacket.dialog(), NoticeDialog.DEFAULT_ACTION",
            "bytecode_source_command", "_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundShowDialogPacket net.minecraft.server.dialog.Dialog net.minecraft.server.dialog.NoticeDialog net.minecraft.server.dialog.CommonDialogData net.minecraft.server.dialog.ActionButton net.minecraft.server.dialog.CommonButtonData net.minecraft.network.protocol.common.CommonPacketTypes net.minecraft.network.protocol.configuration.ConfigurationProtocols"
        ));

        Map<String, Object> answerBody = new LinkedHashMap<>();
        answerBody.put("state", "Configuration");
        answerBody.put("flow", "Clientbound");
        answerBody.put("packet_type", "minecraft:show_dialog");
        answerBody.put("decoded_packet_type", decodedPacket.type().id().toString());
        answerBody.put("decoded_packet_class", decodedPacket.getClass().getName());
        answerBody.put("input_fixture", "Holder.direct(new NoticeDialog(CommonDialogData literal title, NoticeDialog.DEFAULT_ACTION))");
        answerBody.put("input_dialog_class", dialog.getClass().getName());
        answerBody.put("decoded_dialog_class", decodedDialog.getClass().getName());
        answerBody.put("input_dialog_title", common.title().getString());
        answerBody.put("decoded_dialog_title", decodedDialog.common().title().getString());
        answerBody.put("input_dialog_body_count", common.body().size());
        answerBody.put("decoded_dialog_body_count", decodedDialog.common().body().size());
        answerBody.put("input_dialog_input_count", common.inputs().size());
        answerBody.put("decoded_dialog_input_count", decodedDialog.common().inputs().size());
        answerBody.put("input_can_close_with_escape", common.canCloseWithEscape());
        answerBody.put("decoded_can_close_with_escape", decodedDialog.common().canCloseWithEscape());
        answerBody.put("input_pause", common.pause());
        answerBody.put("decoded_pause", decodedDialog.common().pause());
        answerBody.put("input_after_action", common.afterAction().getSerializedName());
        answerBody.put("decoded_after_action", decodedDialog.common().afterAction().getSerializedName());
        answerBody.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        answerBody.put("encoded_body_hex", HexFormat.of().formatHex(body));
        answerBody.put("remaining_after_official_decode", framedIn.readableBytes());
        answerBody.put("configuration_clientbound_packet_table", configurationClientboundPackets);
        answer.put("answer", answerBody);
        return answer;
    }
}
