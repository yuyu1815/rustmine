package dev.rustmine.oracle;

import com.google.gson.JsonObject;
import java.nio.file.Path;
import java.util.Map;
import static dev.rustmine.oracle.OracleJson.readJson;
import static dev.rustmine.oracle.OracleJson.writeAnswer;


public final class OracleHarness {
    private OracleHarness() {
    }

    public static void main(String[] args) throws Exception {
        if (args.length == 0) {
            throw new IllegalArgumentException("expected at least one oracle case path");
        }

        SharedBootstrap.boot();
        for (String arg : args) {
            runCase(Path.of(arg));
        }
    }

    private static void runCase(Path casePath) throws Exception {
        JsonObject input = readJson(casePath);
        String caseId = input.get("case_id").getAsString();

        Map<String, Object> answer = OracleCases.generate(caseId, input);
        writeAnswer(input, answer);
    }
}
