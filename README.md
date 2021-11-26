[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/ssomers/bron-kerbosch?svg=true&branch=master)](https://ci.appveyor.com/project/ssomers/bron-kerbosch)

## What is this?

Performance comparison of various implementations of three Bron-Kerbosch algorithms to find all maximal cliques in a graph.
The purpose is not only to compare the algorithms, but also programming languages, library choices, and the effect of optimization.
Compared to the original project this is forked from, the code is:
* converted from python 2 to python 3.8
* (hopefully) clarified and type safe
* extended with variations on the algorithms
* extended with unit tests and a performance test on random graphs
* most of in Rust, Java, Go, C++ and partly in C# and Scala


## Local optimization

Let's first get one thing out of the way: what does some "obvious" local optimization yield in the simplest, naive Bron-Kerbosch algorithm, in Python and Rust. You might call this premature optimization or low hanging fruit.

* **Ver0:** Ver1 in the original project
* **Ver1:** Same locally optimized, without changing the algorithm as such.

In particular:
  - In the loop, don't calculate the intersection of excluded vertices when we know the intersection of candidates is empty.
  - In Rust, compile a `Clique` from the call stack, instead of passing it around on the heap. Basically just showing off Rust's ability to guarantee, at compile time, this can be done.

### Results

We get almost as much gain as switching programming languages.
[![Time spent on graphs of order 100](https://plotly.com/~stein.somers/774.png "View interactively")](https://plotly.com/~stein.somers/774/)

## Comparing Algorithms

These are all single-threaded implementations (using only one CPU core), all with basic optimalization.

* **Ver1:** Original naive Bron-Kerbosch algorithm Ver1
* **Ver2:** Ver1 excluding neighbours of a pivot that is chosen arbitrarily (optimized original Ver2)
* **Ver2-G:** Similar but with pivot of highest degree in the whole graph, chosen from candidates only
* **Ver2-GP:** Similar but with pivot of highest degree towards the remaining candidates, chosen from candidates only (IK\_GP)
* **Ver2-GPX:** Similar but with pivot of highest degree towards the remaining candidates, chosen from both candidates and excluded (IK\_GPX)
* **Ver2-RP:** Similar but but with pivot randomly chosen from candidates (IK\_RP)
* **Ver3:** Ver2 with degeneracy ordering (optimized, where the original clearly marked it necessary)
* **Ver3-GP:** Ver2-GP with degeneracy ordering, with pivot chosen from candidates only (IK\_GP)
* **Ver3-GPX:** Ver2-GPX with degeneracy ordering, with pivot chosen from both candidates and excluded (IK\_GPX)

### Results

* Ver1 indeed struggles with dense graphs:
[![Time spent on graphs of order 100](https://plotly.com/~stein.somers/783.png "View interactively")](https://plotly.com/~stein.somers/783/)
* Among Ver2 variants, GP and GPX are indeed best…
[![Time spent on graphs of order 100](https://plotly.com/~stein.somers/823.png "View interactively")](https://plotly.com/~stein.somers/823/)
[![Time spent on graphs of order 100](https://plotly.com/~stein.somers/836.png "View interactively")](https://plotly.com/~stein.somers/836/)
…but GPX looses ground in big graphs:
[![Time spent on graphs of order 10k](https://plotly.com/~stein.somers/825.png "View interactively")](https://plotly.com/~stein.somers/825/)
[![Time spent on graphs of order 10k](https://plotly.com/~stein.somers/839.png "View interactively")](https://plotly.com/~stein.somers/839/)
* Ver3 isn't better in Python:
[![Time spent on graphs of order 100](https://plotly.com/~stein.somers/855.png "View interactively")](https://plotly.com/~stein.somers/855/)
[![Time spent on graphs of order 10k](https://plotly.com/~stein.somers/858.png "View interactively")](https://plotly.com/~stein.somers/858/)
[![Time spent on graphs of order 1M](https://plotly.com/~stein.somers/862.png "View interactively")](https://plotly.com/~stein.somers/862/)
* Neither in Rust:
[![Time spent on graphs of order 100](https://plotly.com/~stein.somers/867.png "View interactively")](https://plotly.com/~stein.somers/867/)
[![Time spent on graphs of order 10k](https://plotly.com/~stein.somers/869.png "View interactively")](https://plotly.com/~stein.somers/869/)
* Ver3-GP seems better in Java, at least in bigger graphs:
[![Time spent on graphs of order 100](https://plotly.com/~stein.somers/873.png "View interactively")](https://plotly.com/~stein.somers/873/)
[![Time spent on graphs of order 10k](https://plotly.com/~stein.somers/875.png "View interactively")](https://plotly.com/~stein.somers/875/)
* Same in Go:
[![Time spent on graphs of order 100](https://plotly.com/~stein.somers/885.png "View interactively")](https://plotly.com/~stein.somers/885/)
[![Time spent on graphs of order 10k](https://plotly.com/~stein.somers/887.png "View interactively")](https://plotly.com/~stein.somers/887/)
* And in C#:
[![Time spent on graphs of order 100](https://plotly.com/~stein.somers/879.png "View interactively")](https://plotly.com/~stein.somers/879/)
[![Time spent on graphs of order 10k](https://plotly.com/~stein.somers/881.png "View interactively")](https://plotly.com/~stein.somers/881/)
[![Time spent on graphs of order 1M](https://plotly.com/~stein.somers/883.png "View interactively")](https://plotly.com/~stein.somers/883/)

## Introducing parallelism

These are all implementations of **Ver3-GP** that also exploit parallellism (using all CPU cores).

* **Ver3=GPc:** (Rust, C++, Java) using something resembling channels
* **Ver3=GPs:** (C#, Java, Scala) using simple composition (async, stream, future)
* **Ver3=GP0:** (Go only) channels & 2 + 1 goroutines
* **Ver3=GP1:** (Go only) channels & 2 + 4 goroutines
* **Ver3=GP2:** (Go only) channels & 2 + 16 goroutine
* **Ver3=GP3:** (Go only) channels & 2 + 64 goroutines
* **Ver3=GP4:** (Go only) channels & 2 + 256 goroutines

## Set data structures

All algorithms operate heavily on and with sets. In some languages, it's easy to pick among
various generic set implementations and compare their performance.

### Rust
* **BTreeSet:** std::collections::BTreeSet
* **HashSet:** std::collections::HashSet, a wrapper around hashbrown
* **hashbrown:** HashSet from [crate hashbrown](https://crates.io/crates/hashbrown) 0.11
* **fnv:** FnvHashSet from [crate fnv](https://crates.io/crates/fnv) 1.0
* **ord_vec:** ordered std::collections::Vec (obviously, this can only work well on small graphs)

### C++
* **std_set:** std::set
* **hashset:** std::unordered_set
* **ord_vec:** ordered std::vector (obviously, this can only work well on small graphs)

## Detailed Results

Graphs of the amount of time spent on a particular machine with 6 core CPU,
all on predetermined random graphs (generated with typical pseudo-random generators in python):

* [Dense graphs of order 100](results_100.md): Ver1 indeed can't cope.
* [Graphs of order 10k](results_10k.md): probably the most realistic case.
* [Graphs of order 1M](results_1M.md): who scales best?

Order of a graph = number of vertices.

## Run & Test

### Python 3

    cd python3
    (once) python -m venv venv
    venv\Scripts\activate.bat
    (once) pip install pytest chart-studio hypothesis mypy
    mypy . --ignore-missing-imports
    pytest
    python -O test_maximal_cliques.py

### Rust

    cd rust
    cargo clippy --workspace
    cargo test --workspace
    cargo run --release

### Go

    cd go
    go vet BronKerbosch/...
    go test BronKerbosch/...
    go test -race BronKerbosch/lib
    go run BronKerbosch/main
    python ..\python3\publish.py go 100 10k 1M

### C#
  - open csharp\BronKerboschStudy.sln with Visual Studio 2019
  - set configuration to Debug
  - Test > Run > All Tests
  - set configuration to Release
  - Debug > Start Without Debugging

and finally

    python python3\publish.py c# 100 10k 1M

### C++ 17
Either:
  - open cpp17\BronKerboschStudy.sln with Visual Studio 2019
  - set configuration to Debug
  - Test > Run > All Tests
  - set configuration to Release
  - Debug > Start Without Debugging

or in Mingw-64:

    pacman -S mingw-w64-x86_64-gcc
    PATH="/mingw64/bin:$PATH"
    g++ -DNDEBUG -I. -O -std=c++17 -Wall -Wno-maybe-uninitialized BronKerbosch/*.cpp BronKerboschStudy/*.cpp
    ./a

and finally

    python python3\publish.py c++ 100 10k 1M

### Java
  - open folder java with IntelliJ IDEA 2020.2 (Community Edition)
  - set run configuration to "Test"
  - Run > Run 'Test'
  - set run configuration to "Main"
  - Run > Run 'Main'

and finally

    python python3\publish.py java 100 10k 1M

### Scala
  - open folder scala with IntelliJ IDEA 2021.1 (Community Edition)
  - View > Tool Windows > sbt; Reload sbt Project (or Reload All sbt Projects)
  - enable assertions: comment out `"-Xdisable-assertions"` in build.sbt
  - Build > Rebuild Project
  - set run configuration to test
  - Run > Run 'test'
  - disable assertions: uncomment `"-Xdisable-assertions"` in build.sbt
  - Build > Rebuild Project
  - set run configuration to main
  - Run > Run 'main'

and finally

    python python3\publish.py scala 100 10k 1M

## Context

[More information on Wikipedia](http://en.wikipedia.org/wiki/Bron-Kerbosch_algorithm).

Some algorithm variants (IK_*) are described in the 2008 paper by F. Cazals & C. Karande, “A note on the problem of reporting maximal cliques”, Theoretical Computer Science, 407 (1): 564–568, doi:10.1016/j.tcs.2008.05.010.

## License

[BSD License](http://opensource.org/licenses/BSD-3-Clause)
