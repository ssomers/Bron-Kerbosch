package be.steinsomers.bron_kerbosch;

import lombok.RequiredArgsConstructor;

import java.util.function.Consumer;

@RequiredArgsConstructor
public final class CliqueConsumer {
    public final int minSize;
    private final Consumer<int[]> acceptor;

    public void accept(int[] clique) {
        acceptor.accept(clique);
    }
}

