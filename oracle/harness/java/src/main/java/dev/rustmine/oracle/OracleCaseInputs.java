package dev.rustmine.oracle;

import com.google.gson.JsonObject;
import java.util.ArrayList;
import java.util.List;
import java.util.UUID;


public final class OracleCaseInputs {
    private OracleCaseInputs() {
    }

    public static int[] jsonIntArray(JsonObject object, String fieldName) {
        var jsonArray = object.getAsJsonArray(fieldName);
        int[] values = new int[jsonArray.size()];
        for (int i = 0; i < jsonArray.size(); i += 1) {
            values[i] = jsonArray.get(i).getAsInt();
        }
        return values;
    }

    public static List<UUID> jsonUuidList(JsonObject object, String fieldName) {
        var jsonArray = object.getAsJsonArray(fieldName);
        List<UUID> values = new ArrayList<>();
        for (int i = 0; i < jsonArray.size(); i += 1) {
            values.add(UUID.fromString(jsonArray.get(i).getAsString()));
        }
        return values;
    }
}
