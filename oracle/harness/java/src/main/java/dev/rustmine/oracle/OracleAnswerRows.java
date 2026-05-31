package dev.rustmine.oracle;

import it.unimi.dsi.fastutil.ints.IntList;
import java.util.ArrayList;
import java.util.HexFormat;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.UUID;
import net.minecraft.network.protocol.Packet;
import net.minecraft.resources.Identifier;
import net.minecraft.server.ServerLinks;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.packs.repository.KnownPack;
import static dev.rustmine.oracle.OracleBuffers.bytesAfterVarIntPrefix;


public final class OracleAnswerRows {
    private OracleAnswerRows() {
    }

    public static Map<String, Object> finishDirectionAnswer(
        String flow,
        String packetType,
        Packet<?> decodedPacket,
        boolean instanceTerminal,
        boolean decodedTerminal,
        byte[] framed,
        int remainingAfterDecode,
        List<Map<String, Object>> packetTable
    ) {
        Map<String, Object> row = new LinkedHashMap<>();
        row.put("flow", flow);
        row.put("packet_type", packetType);
        row.put("decoded_packet_type", decodedPacket.type().id().toString());
        row.put("decoded_packet_class", decodedPacket.getClass().getName());
        row.put("instance_is_terminal", instanceTerminal);
        row.put("decoded_is_terminal", decodedTerminal);
        row.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        row.put("encoded_body_hex", HexFormat.of().formatHex(bytesAfterVarIntPrefix(framed)));
        row.put("remaining_after_official_decode", remainingAfterDecode);
        row.put("configuration_packet_table", packetTable);
        return row;
    }

    public static Map<String, Object> framedDirectionAnswer(
        String flow,
        String packetType,
        Packet<?> decodedPacket,
        int inputId,
        int decodedId,
        byte[] framed,
        byte[] body,
        int remainingAfterDecode,
        List<Map<String, Object>> packetTable
    ) {
        Map<String, Object> row = new LinkedHashMap<>();
        row.put("flow", flow);
        row.put("packet_type", packetType);
        row.put("decoded_packet_type", decodedPacket.type().id().toString());
        row.put("decoded_packet_class", decodedPacket.getClass().getName());
        row.put("input_id", inputId);
        row.put("encoded_framed_hex", HexFormat.of().formatHex(framed));
        row.put("encoded_body_hex", HexFormat.of().formatHex(body));
        row.put("decoded_id", decodedId);
        row.put("remaining_after_official_decode", remainingAfterDecode);
        row.put("configuration_packet_table", packetTable);
        return row;
    }

    public static Map<String, Object> clientInformationAnswer(ClientInformation information) {
        Map<String, Object> row = new LinkedHashMap<>();
        row.put("language", information.language());
        row.put("view_distance", information.viewDistance());
        row.put("chat_visibility", information.chatVisibility().name());
        row.put("chat_colors", information.chatColors());
        row.put("model_customisation", information.modelCustomisation());
        row.put("main_hand", information.mainHand().name());
        row.put("text_filtering_enabled", information.textFilteringEnabled());
        row.put("allows_listing", information.allowsListing());
        row.put("particle_status", information.particleStatus().name());
        return row;
    }

    public static List<Map<String, Object>> knownPackAnswers(List<KnownPack> knownPacks) {
        List<Map<String, Object>> rows = new ArrayList<>();
        for (KnownPack knownPack : knownPacks) {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("namespace", knownPack.namespace());
            row.put("id", knownPack.id());
            row.put("version", knownPack.version());
            row.put("is_vanilla", knownPack.isVanilla());
            rows.add(row);
        }
        return rows;
    }

    public static List<Map<String, Object>> serverLinkAnswers(List<ServerLinks.UntrustedEntry> links) {
        List<Map<String, Object>> rows = new ArrayList<>();
        for (ServerLinks.UntrustedEntry link : links) {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("link", link.link());
            row.put("type", link.type().map(knownType -> {
                Map<String, Object> known = new LinkedHashMap<>();
                known.put("kind", "known");
                known.put("name", knownType.name());
                return known;
            }, customName -> {
                Map<String, Object> custom = new LinkedHashMap<>();
                custom.put("kind", "custom");
                custom.put("text", customName.getString());
                return custom;
            }));
            rows.add(row);
        }
        return rows;
    }

    public static List<String> identifierStrings(Set<Identifier> identifiers) {
        return identifiers.stream().map(Identifier::toString).sorted().toList();
    }

    public static List<String> uuidStrings(List<UUID> values) {
        return values.stream().map(UUID::toString).toList();
    }

    public static List<Integer> intListValues(IntList values) {
        List<Integer> result = new ArrayList<>();
        for (int i = 0; i < values.size(); i += 1) {
            result.add(values.getInt(i));
        }
        return result;
    }
}
