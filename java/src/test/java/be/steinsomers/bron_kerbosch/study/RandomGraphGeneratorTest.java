package be.steinsomers.bron_kerbosch.study;

import org.junit.jupiter.api.Test;

import java.util.Random;

final class RandomGraphGeneratorTest {
    @Test
    void random_undirected_graph() {
        var rng = new Random(19680516);
        var gen = new RandomGraphGenerator(rng);
        gen.newUndirected(3, 0);
        gen.newUndirected(3, 1);
        gen.newUndirected(3, 2);
        gen.newUndirected(4, 0);
        gen.newUndirected(4, 1);
        gen.newUndirected(4, 2);
        gen.newUndirected(4, 3);
        gen.newUndirected(4, 4);
        gen.newUndirected(4, 5);
    }
}
