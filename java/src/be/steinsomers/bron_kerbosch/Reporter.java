package be.steinsomers.bron_kerbosch;

import java.util.List;

interface Reporter {
    void record(List<Integer> clique);
}
