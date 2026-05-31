package dev.rustmine.oracle;

import java.lang.reflect.Constructor;
import net.minecraft.core.BlockPos;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.network.protocol.common.custom.BrandPayload;
import net.minecraft.network.protocol.game.ClientboundBlockEntityDataPacket;
import net.minecraft.world.level.block.entity.BlockEntityType;


public final class OraclePacketConstruction {
    private OraclePacketConstruction() {
    }

    public static BrandPayload requireBrandPayload(Object payload) {
        if (payload instanceof BrandPayload brandPayload) {
            return brandPayload;
        }
        throw new IllegalStateException("expected BrandPayload, got " + payload.getClass().getName());
    }

    public static ClientboundBlockEntityDataPacket constructBlockEntityDataPacket(
        BlockPos pos,
        BlockEntityType<?> type,
        CompoundTag tag
    ) {
        try {
            Constructor<ClientboundBlockEntityDataPacket> constructor =
                ClientboundBlockEntityDataPacket.class.getDeclaredConstructor(
                    BlockPos.class,
                    BlockEntityType.class,
                    CompoundTag.class
                );
            constructor.setAccessible(true);
            return constructor.newInstance(pos, type, tag);
        } catch (ReflectiveOperationException err) {
            throw new IllegalStateException("failed to call official ClientboundBlockEntityDataPacket constructor", err);
        }
    }
}
