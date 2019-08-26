package be.steinsomers.bron_kerbosch;

import java.util.List;
import java.util.HashSet;

public final class UndirectedGraph {
    private List<HashSet<Integer>> itsAdjacencies;

    public UndirectedGraph(List<HashSet<Integer>> adjacencies) {
        for (int v = 0; v < adjacencies.size(); ++v) {
            for (int w : adjacencies.get(v)) {
                assert v != w;
                assert adjacencies.get(w).contains(v);
            }
        }
        itsAdjacencies = adjacencies;
    }

    public int order() {
        return itsAdjacencies.size();
    }

    public int size() {
        var total = 0;
        for (var v = 0; v < order(); ++v)
            total += degree(v);
        assert total % 2 == 0;
        return total / 2;
    }

    public HashSet<Integer> neighbours(int node) {
        return itsAdjacencies.get(node);
    }

    public int degree(int node) {
        return itsAdjacencies.get(node).size();
    }

    public HashSet<Integer> connectedVertices() {
        var result = new HashSet<Integer>();
        for (var v = 0; v < order(); ++v)
            if (degree(v) > 0)
                result.add(v);
        return result;
    }
}
