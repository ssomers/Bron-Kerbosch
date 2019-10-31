package be.steinsomers.bron_kerbosch;

import java.util.Collection;

@FunctionalInterface
interface Reporter {
    void record(Collection<Integer> clique);
}
