package dev.rustmine.oracle;

import net.minecraft.SharedConstants;
import net.minecraft.server.Bootstrap;


final class SharedBootstrap {
    private SharedBootstrap() {
    }

    static void boot() {
        SharedConstants.tryDetectVersion();
        Bootstrap.bootStrap();
    }
}
