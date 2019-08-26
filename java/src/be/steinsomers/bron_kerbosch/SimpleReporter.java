package be.steinsomers.bron_kerbosch;

import java.util.ArrayList;
import java.util.List;

public final class SimpleReporter implements Reporter {
    public ArrayList<List<Integer>> cliques = new ArrayList<>();

    public void record(List<Integer> clique) {
        assert clique.size() > 1;
        cliques.add(clique);
    }
}
