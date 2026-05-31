package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import net.minecraft.core.Holder;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.FriendlyByteBuf;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.common.ClientboundShowDialogPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.server.dialog.CommonDialogData;
import net.minecraft.server.dialog.Dialog;
import net.minecraft.server.dialog.DialogAction;
import net.minecraft.server.dialog.NoticeDialog;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;


public final class PlayShowDialogClientboundFramedDispatchCase {
    private PlayShowDialogClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        CommonDialogData common = new CommonDialogData(
            Component.literal("Oracle play notice"),
            Optional.empty(),
            true,
            false,
            DialogAction.CLOSE,
            List.of(),
            List.of()
        );
        Dialog dialog = new NoticeDialog(common, NoticeDialog.DEFAULT_ACTION);
        ClientboundShowDialogPacket packet = new ClientboundShowDialogPacket(Holder.direct(dialog));

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundShowDialogPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:show_dialog");

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
        if (!(decodedPacket instanceof ClientboundShowDialogPacket decodedShowDialog)) {
            throw new IllegalStateException(
                "decoded Play show_dialog as unexpected packet " + decodedPacket.getClass().getName()
            );
        }
        Dialog decodedDialog = decodedShowDialog.dialog().value();

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundShowDialogPacket(Holder.direct(NoticeDialog)); ClientboundShowDialogPacket.STREAM_CODEC; Dialog.STREAM_CODEC; ByteBufCodecs.holder direct branch; NoticeDialog(CommonDialogData, NoticeDialog.DEFAULT_ACTION); GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundShowDialogPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.common.ClientboundShowDialogPacket net.minecraft.server.dialog.Dialog net.minecraft.server.dialog.NoticeDialog net.minecraft.server.dialog.CommonDialogData net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:show_dialog",
            decodedPacket,
            "official ClientboundShowDialogPacket(Holder.direct(NoticeDialog)) Play context-free fixture",
            "direct Dialog holder marker plus direct NoticeDialog body",
            packetId,
            0,
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
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
        answer.put("answer", answerBody);
        return answer;
    }
}
