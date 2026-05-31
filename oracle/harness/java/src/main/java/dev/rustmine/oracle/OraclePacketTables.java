package dev.rustmine.oracle;

import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.network.protocol.game.GameProtocols;


public final class OraclePacketTables {
    private OraclePacketTables() {
    }

    public static List<Map<String, Object>> playClientboundPacketTable() {
        List<Map<String, Object>> rows = new ArrayList<>();
        GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets((packetType, packetId) -> {
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("packet_id", packetId);
            row.put("packet_type", packetType.id().toString());
            row.put("flow", packetType.flow().id());
            rows.add(row);
        });
        return rows;
    }

    public static int requirePacketId(List<Map<String, Object>> packetTable, String packetType) {
        for (Map<String, Object> row : packetTable) {
            if (packetType.equals(row.get("packet_type"))) {
                return (Integer) row.get("packet_id");
            }
        }
        throw new IllegalStateException("missing official packet id for " + packetType);
    }
}
