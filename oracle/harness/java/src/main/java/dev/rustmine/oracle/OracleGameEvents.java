package dev.rustmine.oracle;

import net.minecraft.network.protocol.game.ClientboundGameEventPacket;
import static dev.rustmine.oracle.OracleReflection.privateInt;


public final class OracleGameEvents {
    private OracleGameEvents() {
    }

    public static ClientboundGameEventPacket.Type gameEventType(String eventName) {
        return switch (eventName) {
            case "NO_RESPAWN_BLOCK_AVAILABLE" -> ClientboundGameEventPacket.NO_RESPAWN_BLOCK_AVAILABLE;
            case "START_RAINING" -> ClientboundGameEventPacket.START_RAINING;
            case "STOP_RAINING" -> ClientboundGameEventPacket.STOP_RAINING;
            case "CHANGE_GAME_MODE" -> ClientboundGameEventPacket.CHANGE_GAME_MODE;
            case "WIN_GAME" -> ClientboundGameEventPacket.WIN_GAME;
            case "DEMO_EVENT" -> ClientboundGameEventPacket.DEMO_EVENT;
            case "PLAY_ARROW_HIT_SOUND" -> ClientboundGameEventPacket.PLAY_ARROW_HIT_SOUND;
            case "RAIN_LEVEL_CHANGE" -> ClientboundGameEventPacket.RAIN_LEVEL_CHANGE;
            case "THUNDER_LEVEL_CHANGE" -> ClientboundGameEventPacket.THUNDER_LEVEL_CHANGE;
            case "PUFFER_FISH_STING" -> ClientboundGameEventPacket.PUFFER_FISH_STING;
            case "GUARDIAN_ELDER_EFFECT" -> ClientboundGameEventPacket.GUARDIAN_ELDER_EFFECT;
            case "IMMEDIATE_RESPAWN" -> ClientboundGameEventPacket.IMMEDIATE_RESPAWN;
            case "LIMITED_CRAFTING" -> ClientboundGameEventPacket.LIMITED_CRAFTING;
            case "LEVEL_CHUNKS_LOAD_START" -> ClientboundGameEventPacket.LEVEL_CHUNKS_LOAD_START;
            default -> throw new IllegalArgumentException("unsupported game event fixture " + eventName);
        };
    }

    public static String gameEventName(ClientboundGameEventPacket.Type event) {
        if (event == ClientboundGameEventPacket.NO_RESPAWN_BLOCK_AVAILABLE) {
            return "NO_RESPAWN_BLOCK_AVAILABLE";
        }
        if (event == ClientboundGameEventPacket.START_RAINING) {
            return "START_RAINING";
        }
        if (event == ClientboundGameEventPacket.STOP_RAINING) {
            return "STOP_RAINING";
        }
        if (event == ClientboundGameEventPacket.CHANGE_GAME_MODE) {
            return "CHANGE_GAME_MODE";
        }
        if (event == ClientboundGameEventPacket.WIN_GAME) {
            return "WIN_GAME";
        }
        if (event == ClientboundGameEventPacket.DEMO_EVENT) {
            return "DEMO_EVENT";
        }
        if (event == ClientboundGameEventPacket.PLAY_ARROW_HIT_SOUND) {
            return "PLAY_ARROW_HIT_SOUND";
        }
        if (event == ClientboundGameEventPacket.RAIN_LEVEL_CHANGE) {
            return "RAIN_LEVEL_CHANGE";
        }
        if (event == ClientboundGameEventPacket.THUNDER_LEVEL_CHANGE) {
            return "THUNDER_LEVEL_CHANGE";
        }
        if (event == ClientboundGameEventPacket.PUFFER_FISH_STING) {
            return "PUFFER_FISH_STING";
        }
        if (event == ClientboundGameEventPacket.GUARDIAN_ELDER_EFFECT) {
            return "GUARDIAN_ELDER_EFFECT";
        }
        if (event == ClientboundGameEventPacket.IMMEDIATE_RESPAWN) {
            return "IMMEDIATE_RESPAWN";
        }
        if (event == ClientboundGameEventPacket.LIMITED_CRAFTING) {
            return "LIMITED_CRAFTING";
        }
        if (event == ClientboundGameEventPacket.LEVEL_CHUNKS_LOAD_START) {
            return "LEVEL_CHUNKS_LOAD_START";
        }
        throw new IllegalArgumentException("unknown game event type id " + privateInt(event, "id"));
    }
}
