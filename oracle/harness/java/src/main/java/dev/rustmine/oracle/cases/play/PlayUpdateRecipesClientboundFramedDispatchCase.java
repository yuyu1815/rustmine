package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import io.netty.buffer.Unpooled;
import java.util.List;
import java.util.Map;
import net.minecraft.core.RegistryAccess;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.protocol.Packet;
import net.minecraft.network.protocol.game.ClientboundUpdateRecipesPacket;
import net.minecraft.network.protocol.game.GameProtocols;
import net.minecraft.world.item.crafting.SelectableRecipe;
import net.minecraft.world.item.crafting.StonecutterRecipe;
import static dev.rustmine.oracle.OracleBuffers.*;
import static dev.rustmine.oracle.OraclePacketTables.*;
import static dev.rustmine.oracle.OraclePlayAnswers.*;

public final class PlayUpdateRecipesClientboundFramedDispatchCase {
    private PlayUpdateRecipesClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        ClientboundUpdateRecipesPacket packet = new ClientboundUpdateRecipesPacket(
            Map.of(),
            SelectableRecipe.SingleInputSet.<StonecutterRecipe>empty()
        );

        RegistryAccess registryAccess = RegistryAccess.EMPTY;
        RegistryFriendlyByteBuf fixtureBodyOut =
            new RegistryFriendlyByteBuf(Unpooled.buffer(), registryAccess);
        ClientboundUpdateRecipesPacket.STREAM_CODEC.encode(fixtureBodyOut, packet);
        byte[] fixtureBody = readableBytes(fixtureBodyOut);

        RegistryFriendlyByteBuf packetIn =
            new RegistryFriendlyByteBuf(Unpooled.wrappedBuffer(fixtureBody), registryAccess);
        ClientboundUpdateRecipesPacket streamDecoded =
            ClientboundUpdateRecipesPacket.STREAM_CODEC.decode(packetIn);

        List<Map<String, Object>> playClientboundPackets = playClientboundPacketTable();
        int packetId = requirePacketId(playClientboundPackets, "minecraft:update_recipes");

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
        if (!(decodedPacket instanceof ClientboundUpdateRecipesPacket decodedUpdateRecipes)) {
            throw new IllegalStateException(
                "decoded Play update_recipes as unexpected packet " + decodedPacket.getClass().getName()
            );
        }

        Map<String, Object> answer = playAnswerHeader(
            input,
            "ClientboundUpdateRecipesPacket(Map<ResourceKey<RecipePropertySet>, RecipePropertySet>, SelectableRecipe.SingleInputSet<StonecutterRecipe>); RecipePropertySet.STREAM_CODEC empty map branch; SelectableRecipe.SingleInputSet.empty(); SelectableRecipe.SingleInputSet.noRecipeCodec() empty list branch; GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...); GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundUpdateRecipesPacket)",
            "CP=\"_analysis/minecraft-26.1.2/client.jar:$(cat oracle/harness/java/build/classpath.txt)\"; _tools/java/jdk-25-full/Contents/Home/bin/javap -classpath \"$CP\" -c -p net.minecraft.network.protocol.game.ClientboundUpdateRecipesPacket net.minecraft.world.item.crafting.SelectableRecipe net.minecraft.world.item.crafting.SelectableRecipe\\$SingleInputSet net.minecraft.world.item.crafting.RecipePropertySet net.minecraft.network.protocol.game.GameProtocols net.minecraft.network.protocol.game.GamePacketTypes"
        );
        Map<String, Object> answerBody = playAnswerBody(
            "minecraft:update_recipes",
            decodedPacket,
            "official ClientboundUpdateRecipesPacket fixture with empty itemSets map and SelectableRecipe.SingleInputSet.empty(); no item, recipe display, or recipe property payload is entered",
            "empty recipe property-set map followed by empty stonecutter single-input recipe list",
            packetId,
            packetIn.readableBytes(),
            framed,
            body,
            fixtureBody,
            framedIn.readableBytes(),
            playClientboundPackets
        );
        answerBody.put("input_item_set_count", packet.itemSets().size());
        answerBody.put("stream_decoded_item_set_count", streamDecoded.itemSets().size());
        answerBody.put("decoded_item_set_count", decodedUpdateRecipes.itemSets().size());
        answerBody.put("input_stonecutter_recipe_count", packet.stonecutterRecipes().entries().size());
        answerBody.put("stream_decoded_stonecutter_recipe_count", streamDecoded.stonecutterRecipes().entries().size());
        answerBody.put("decoded_stonecutter_recipe_count", decodedUpdateRecipes.stonecutterRecipes().entries().size());
        answer.put("answer", answerBody);
        return answer;
    }
}
