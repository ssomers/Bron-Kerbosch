package be.steinsomers.bron_kerbosch;

import java.util.ArrayDeque;
import java.util.Collection;

public final class SimpleReporter implements Reporter {
    public final Collection<Collection<Integer>> cliques = new ArrayDeque<>();

    public void record(Collection<Integer> clique) {
        assert clique.size() > 1;
        cliques.add(clique);
    }
}
