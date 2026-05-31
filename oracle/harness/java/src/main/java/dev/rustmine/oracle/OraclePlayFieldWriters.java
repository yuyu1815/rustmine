package dev.rustmine.oracle;

import java.util.Map;
import net.minecraft.network.protocol.game.ClientboundEntityPositionSyncPacket;
import net.minecraft.network.protocol.game.ClientboundInitializeBorderPacket;
import net.minecraft.network.protocol.game.ClientboundLevelEventPacket;
import net.minecraft.network.protocol.game.ClientboundMoveEntityPacket;
import net.minecraft.network.protocol.game.ClientboundMoveVehiclePacket;
import net.minecraft.network.protocol.game.ClientboundOpenBookPacket;
import net.minecraft.network.protocol.game.ClientboundPlayerAbilitiesPacket;
import static dev.rustmine.oracle.OracleReflection.privateByte;
import static dev.rustmine.oracle.OracleReflection.privateInt;


public final class OraclePlayFieldWriters {
    private OraclePlayFieldWriters() {
    }

    public static void putEntityPositionSyncFields(
        Map<String, Object> answerBody,
        String prefix,
        ClientboundEntityPositionSyncPacket packet
    ) {
        answerBody.put(prefix + "_entity_id", packet.id());
        answerBody.put(prefix + "_x", packet.values().position().x);
        answerBody.put(prefix + "_y", packet.values().position().y);
        answerBody.put(prefix + "_z", packet.values().position().z);
        answerBody.put(prefix + "_delta_x", packet.values().deltaMovement().x);
        answerBody.put(prefix + "_delta_y", packet.values().deltaMovement().y);
        answerBody.put(prefix + "_delta_z", packet.values().deltaMovement().z);
        answerBody.put(prefix + "_y_rot", packet.values().yRot());
        answerBody.put(prefix + "_x_rot", packet.values().xRot());
        answerBody.put(prefix + "_on_ground", packet.onGround());
    }

    public static void putLevelEventFields(
        Map<String, Object> answerBody,
        String prefix,
        ClientboundLevelEventPacket packet
    ) {
        answerBody.put(prefix + "_level_event_type", packet.getType());
        answerBody.put(prefix + "_block_x", packet.getPos().getX());
        answerBody.put(prefix + "_block_y", packet.getPos().getY());
        answerBody.put(prefix + "_block_z", packet.getPos().getZ());
        answerBody.put(prefix + "_data", packet.getData());
        answerBody.put(prefix + "_global_event", packet.isGlobalEvent());
    }

    public static void putMoveEntityFields(
        Map<String, Object> answerBody,
        String prefix,
        ClientboundMoveEntityPacket packet
    ) {
        answerBody.put(prefix + "_entity_id", privateInt(packet, "entityId"));
        answerBody.put(prefix + "_xa", (int) packet.getXa());
        answerBody.put(prefix + "_ya", (int) packet.getYa());
        answerBody.put(prefix + "_za", (int) packet.getZa());
        answerBody.put(prefix + "_move_y_rot_byte", (int) privateByte(packet, "yRot"));
        answerBody.put(prefix + "_move_x_rot_byte", (int) privateByte(packet, "xRot"));
        answerBody.put(prefix + "_y_rot_degrees", packet.getYRot());
        answerBody.put(prefix + "_x_rot_degrees", packet.getXRot());
        answerBody.put(prefix + "_on_ground", packet.isOnGround());
        answerBody.put(prefix + "_has_rotation", packet.hasRotation());
        answerBody.put(prefix + "_has_position", packet.hasPosition());
    }

    public static void putMoveVehicleFields(
        Map<String, Object> answerBody,
        String prefix,
        ClientboundMoveVehiclePacket packet
    ) {
        answerBody.put(prefix + "_x", packet.position().x);
        answerBody.put(prefix + "_y", packet.position().y);
        answerBody.put(prefix + "_z", packet.position().z);
        answerBody.put(prefix + "_y_rot", packet.yRot());
        answerBody.put(prefix + "_x_rot", packet.xRot());
    }

    public static void putOpenBookFields(
        Map<String, Object> answerBody,
        String prefix,
        ClientboundOpenBookPacket packet
    ) {
        answerBody.put(prefix + "_hand", packet.getHand().name());
        answerBody.put(prefix + "_hand_ordinal", packet.getHand().ordinal());
    }

    public static void putPlayerAbilitiesFields(
        Map<String, Object> answerBody,
        String prefix,
        ClientboundPlayerAbilitiesPacket packet
    ) {
        int flags = 0;
        if (packet.isInvulnerable()) {
            flags |= 1;
        }
        if (packet.isFlying()) {
            flags |= 2;
        }
        if (packet.canFly()) {
            flags |= 4;
        }
        if (packet.canInstabuild()) {
            flags |= 8;
        }
        answerBody.put(prefix + "_flags", flags);
        answerBody.put(prefix + "_invulnerable", packet.isInvulnerable());
        answerBody.put(prefix + "_flying", packet.isFlying());
        answerBody.put(prefix + "_can_fly", packet.canFly());
        answerBody.put(prefix + "_instabuild", packet.canInstabuild());
        answerBody.put(prefix + "_flying_speed", packet.getFlyingSpeed());
        answerBody.put(prefix + "_walking_speed", packet.getWalkingSpeed());
    }

    public static void putInitializeBorderFields(
        Map<String, Object> answerBody,
        String prefix,
        double newCenterX,
        double newCenterZ,
        double oldSize,
        double newSize,
        long lerpTime,
        int newAbsoluteMaxSize,
        int warningBlocks,
        int warningTime
    ) {
        answerBody.put(prefix + "_new_center_x", newCenterX);
        answerBody.put(prefix + "_new_center_z", newCenterZ);
        answerBody.put(prefix + "_old_size", oldSize);
        answerBody.put(prefix + "_new_size", newSize);
        answerBody.put(prefix + "_lerp_time", lerpTime);
        answerBody.put(prefix + "_new_absolute_max_size", newAbsoluteMaxSize);
        answerBody.put(prefix + "_warning_blocks", warningBlocks);
        answerBody.put(prefix + "_warning_time", warningTime);
    }

    public static void putInitializeBorderFields(
        Map<String, Object> answerBody,
        String prefix,
        ClientboundInitializeBorderPacket packet
    ) {
        putInitializeBorderFields(
            answerBody,
            prefix,
            packet.getNewCenterX(),
            packet.getNewCenterZ(),
            packet.getOldSize(),
            packet.getNewSize(),
            packet.getLerpTime(),
            packet.getNewAbsoluteMaxSize(),
            packet.getWarningBlocks(),
            packet.getWarningTime()
        );
    }
}
