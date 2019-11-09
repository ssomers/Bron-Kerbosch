[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/ssomers/bron-kerbosch?svg=true&branch=master)](https://ci.appveyor.com/project/ssomers/bron-kerbosch)

## What is this?

Performance comparison of various implementations of three Bron-Kerbosch algorithms to find all maximal cliques in a graph.
The purpose is not only to compare the algorithms, but also programming languages, and the effect of optimization within a programming language.
Compared to the original forked from, the code is:
* converted from python 2 to python 3.7
* (hopefully) clarified and type safe
* extended with variations on the algorithms
* extended with unit tests and a performance test on random graphs
* all that mirrored in Rust, Java, Scala, Go, and partly in C#

## Implementations

* **Ver1:** Naive Bron-Kerbosch algorithm
* **Ver1+:** Ver1 optimized, including language-specific tweaks
* **Ver2+:** Ver1+ excluding neighbours of a pivot that is chosen arbitrarily
* **Ver2+G:** Similar but with pivot of highest degree in the whole graph, chosen from candidates only
* **Ver2+GP:** Similar but with pivot of highest degree towards the remaining candidates, chosen from candidates only (IK\_GP)
* **Ver2+GPX:** Similar but with pivot of highest degree towards the remaining candidates, chosen from both candidates and excluded (IK\_GPX)
* **Ver2+RP:** Similar but but with pivot randomly chosen from candidates (IK\_RP)
* **Ver3+:** Ver2+ with degeneracy ordering (optimized, where the original clearly marked it necessary)
* **Ver3+GP:** Ver2+GP with degeneracy ordering
* **Ver3+GPX:** Ver2+GPX with degeneracy ordering
* **Ver3+MT:** (Rust, Java only) Ver3+GP with multi-threading through channels (2 + 5 threads
* **Ver3+ST:** (Java only) Ver3+GP with simple multi-threading using streams
* **Ver3+GP2:** (Go only) Ver3+GP with multi-threading (2 + 5 goroutines)
* **Ver3+GP3:** (Go only) Ver3+GP with multi-threading (2 + 15 goroutine)
* **Ver3+GP4:** (Go only) Ver3+GP with multi-threading (2 + 45 goroutines)
* **Ver3+GP5:** (Go only) Ver3+GP with multi-threading (2 + 135 goroutines)

## Results

Graphs of the amount of time spent on a particular machine with 6 core CPU,
all on random graphs (generated with typical pseudo-random generators for each language, but results seem consistent hen varying the random seed):

* [Dense graphs of order 100](results_100.md): Ver1 indeed can't cope.
* [Graphs of order 10k](results_10k.md): probably the most realistic case.
* [Very sparse large graphs](results_sparse.md): Ver1 doesn't scale in particular implementations.
* [Graphs of order 1M](results_1M.md): who scales best?

Order of a graph = number of vertices.

## Run & Test

### Run Python 3

    cd python3
    (once) python -m venv venv
    venv\Scripts\activate.bat
    (once) pip install pytest chart-studio
    python -O test_maximal_cliques.py

### Test Python 3

    cd python3
    (once) python -m venv venv
    venv\Scripts\activate.bat
    (once) pip install hypothesis mypy
    mypy . --ignore-missing-imports
    pytest

### Run Rust

    cd rust
    cargo run --release

### Test Rust

    cd rust
    cargo test --all

### Run Go

    set GOPATH=%CD%\go
    go run main
    python python3\publish.py go 100 10k 1M

### Test Go
    
    set GOPATH=%CD%\go
    go vet BronKerbosch main
    go test -race BronKerbosch

### Run C#
  - open csharp\BronKerboschStudy.sln with Visual Studio Community 2017
  - set configuration to Release
  - Debug > Start Without Debugging

and finally

    python python3\publish.py c# 100 10k 999k 1M

### Test C#
  - set configuration to Debug
  - Test > Run > All Tests

### Run Java
  - open java\bron-kerbosch.iml with IntelliJ IDEA 2019.2 (Community Edition)
  - set run configuration to "Main"
  - Run > Run 'Main'

and finally

    python python3\publish.py java 100 10k 1M

### Test Java
  - set run configuration to "Test"
  - Run > Run 'Test'

### Run Scala
  - open scala\build.sbt with IntelliJ IDEA 2019.2 (Community Edition)
  - set compiler configuration to release: open File > Settings > Build, Execution, Deployment > Compiler > Scala Compiler; select Bron-Kerbosch and move to release profile; change something else so IntelliJ doesn't ignore you, Apply (upon which IntelliJ applies the profile change and sometimes the something else), revert the something else and Apply (all this just to compile with -Xdisable-assertions)
  - Build > Rebuild
  - set run configuration to main
  - Run > Run 'main'

and finally

    python python3\publish.py scala 100 10k 1M

### Test Scala
  - set compiler configuration to debug
  - Build > Rebuild
  - set run configuration to test
  - Run > Run 'test'

## Context

[More information on Wikipedia](http://en.wikipedia.org/wiki/Bron-Kerbosch_algorithm).

Some algorithm variants (IK_*) are described in the 2008 paper by F. Cazals & C. Karande, “A note on the problem of reporting maximal cliques”, Theoretical Computer Science, 407 (1): 564–568, doi:10.1016/j.tcs.2008.05.010.

## License

[BSD License](http://opensource.org/licenses/BSD-3-Clause)
