package be.steinsomers.bron_kerbosch.study;

import org.junit.jupiter.api.Test;

import java.util.Random;

class RandomGraphGeneratorTest {
    @Test
    void random_undirected_graph() {
        var rng = new Random(19680516);
        var gen = new RandomGraphGenerator(rng);
        gen.new_undirected(3, 0);
        gen.new_undirected(3, 1);
        gen.new_undirected(3, 2);
        gen.new_undirected(4, 0);
        gen.new_undirected(4, 1);
        gen.new_undirected(4, 2);
        gen.new_undirected(4, 3);
        gen.new_undirected(4, 4);
        gen.new_undirected(4, 5);
    }
}
