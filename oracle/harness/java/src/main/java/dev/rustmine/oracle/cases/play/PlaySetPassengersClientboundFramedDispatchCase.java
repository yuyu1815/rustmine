package dev.rustmine.oracle.cases.play;

import com.google.gson.JsonObject;
import java.util.Map;
import dev.rustmine.oracle.EntityFixturePolicyAnswers;

public final class PlaySetPassengersClientboundFramedDispatchCase {
    private PlaySetPassengersClientboundFramedDispatchCase() {
    }

    public static Map<String, Object> generate(JsonObject input) {
        return EntityFixturePolicyAnswers.generate(input, "minecraft:set_passengers");
    }
}
