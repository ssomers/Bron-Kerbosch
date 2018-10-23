## What is this?

Performance comparison of three Bron–Kerbosch algorithm implementations that find all maximal cliques in a graph.
Compared to the original, converted from python 2 to python 3.7, (hopefully) clarified, extended with big random graphs, and a few unit tests.


## Implementations

* **Ver1:** naive Bron–Kerbosch algorithm
* **Ver2:** Ver1 with pivot
* **Ver3:** Ver2 with degeneracy ordering
* **Ver4:** Ver2 slightly optimized
* **Ver6:** Ver3 recursing

## Run

    python -O test_maximal_cliques.py


## Test
- Manually:
    pytest
- On AppVeyor: [![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/ssomers/bron-kerbosch?svg=true&branch=master)](https://ci.appveyor.com/project/ssomers/bron-kerbosch)


## Time Comlexity

Worst-case time-complexity analysis is [here](http://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm#Worst-case_analysis).

## License

[BSD License](http://opensource.org/licenses/BSD-3-Clause)
