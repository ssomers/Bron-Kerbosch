package be.steinsomers.bron_kerbosch;

import java.util.List;

public interface Reporter {
    void record(List<Integer> clique);
}
