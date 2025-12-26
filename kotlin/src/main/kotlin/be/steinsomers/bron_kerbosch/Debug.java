package be.steinsomers.bron_kerbosch;

import org.jetbrains.annotations.NotNull;

import java.util.function.BooleanSupplier;

public final class Debug {
    public static void Assert(@NotNull BooleanSupplier condition) {
        assert condition.getAsBoolean();
    }
}
