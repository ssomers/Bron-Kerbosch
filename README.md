[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/ssomers/bron-kerbosch?svg=true&branch=master)](https://ci.appveyor.com/project/ssomers/bron-kerbosch)

## What is this?

Performance comparison of various implementations of three
[Bron-Kerbosch algorithms](http://en.wikipedia.org/wiki/Bron-Kerbosch_algorithm)
to find all maximal cliques in a graph.

Some algorithm variants (IK_*) are described in the 2008 paper by F. Cazals & C. Karande,
“A note on the problem of reporting maximal cliques”,
Theoretical Computer Science, 407 (1): 564–568, doi:10.1016/j.tcs.2008.05.010.

The purpose of this fork is not only to compare the algorithms, but also programming languages,
 library choices, and the effect of optimization, chiefly parallelism.

Compared to the original project this is forked from, the code is:
* converted from python 2 to python 3.9
* (hopefully) clarified and type safe
* extended with variations on the algorithms
* extended with unit tests, property based testing, and a performance test on random graphs
* most of in Rust, Java, Go, C++ and partly in C# and Scala

Beware that my Scala knowledge and code is the least developed of all languages.

All charts below show the amount of time spent on the same particular Windows machine with 6 core CPU,
all on the same predetermined random graph, with error bars showing the minimum and maximum
over 5 or 3 samples.
Order of a graph = number of vertices.

## Executive summary
* Better algorithms invented to counter treacherous cases stand their ground on a vanilla random graph.
* Programming language makes a difference, as in factor 2 up to 8.
  - Rust is clearly the fastest, but beware I contributed some performance improvements to its
    collection library, more than I invested in the other, more established languages.
  - C# is the runner up, surpringly (to me).
  - Python is the slowest, not surprisingly.
  - C++ is clearly not the fastest (and I claim this with the confidence of 20 years of professional C++ development).
* Multi-threading helps a lot too, and how programming languages accommodate for it makes a huge difference.
  Python is the worst in that respect, I couldn't get any multi-threading code to work faster than the single-threaded code.
* Collection libraries don't matter much, though hashing reaches sizes a B-tree can only dream of.

## Local optimization

Let's first get one thing out of the way: what does some local optimization yield in the simplest,
naive Bron-Kerbosch algorithm, in Python and Rust. Is this premature optimization or low hanging fruit?

* **Ver0:** Ver1 in the original project
* **Ver1:** Same locally optimized, without changing the algorithm as such.
In particular:
  - In the (many) deepest iterations, when we see the intersection of candidates is empty, don't
    calculate all the nearby excluded vertices, just check if that set is empty or not.
  - In Rust, compile a `Clique` from the call stack, instead of passing it around on the heap.
    Basically showing off Rust's ability to guarantee, at compile time, this can be done safely.

### Results

We gain almost as much as through switching programming languages:
![Time spent on graphs of order 100](doc/report_1.png)

Therefore, all the other implementations will contain similar tweaks.

## Comparing algorithms

* **Ver1:** Naive but optimized Bron-Kerbosch algorithm
* **Ver2:** Ver1 excluding neighbours of a pivot that is chosen arbitrarily (optimized original Ver2)
* **Ver2-G:** Ver2 but pivot is the candidate of the highest degree in the whole graph
* **Ver2-GP:** Ver2 but pivot is the candidate of the highest degree towards the remaining candidates (IK\_GP in the paper)
* **Ver2-GPX:** Ver2-GP but pivot also chosen from excluded vertices (IK\_GPX in the paper)
* **Ver2-RP:** Similar but but with pivot randomly chosen from candidates (IK\_RP in the paper)
* **Ver3:** Ver2 with degeneracy ordering
* **Ver3-GP:** Ver2-GP with degeneracy ordering
* **Ver3-GPX:** Ver2-GPX with degeneracy ordering

These are all single-threaded implementations (using only one CPU core).

### Results

* Ver1 indeed struggles with dense graphs, when it has to cover more than half of the 4950 possible edges
![Time spent on graphs of order 100](doc/report_2.png)

* Among Ver2 variants, GP and GPX are indeed best…
![Time spent on graphs of order 100](doc/report_3_python3_100.png)
![Time spent on graphs of order 100](doc/report_3_java_100.png)

* …but GPX looses ground in big graphs
![Time spent on graphs of order 10k](doc/report_3_python3_10k.png)
![Time spent on graphs of order 10k](doc/report_3_java_10k.png)

* Ver3-GP wins somewhat from Ver2-GP in big graphs…
![Time spent on graphs of order 10k](doc/report_4_java_10k.png)
![Time spent on graphs of order 10k](doc/report_4_go_10k.png)
![Time spent on graphs of order 10k](doc/report_4_rust_10k.png)
![Time spent on graphs of order 10k](doc/report_4_csharp_10k.png)
![Time spent on graphs of order 1M](doc/report_4_csharp_1M.png)

* …except in Python…
![Time spent on graphs of order 10k](doc/report_4_python3_10k.png)
![Time spent on graphs of order 1M](doc/report_4_python3_1M.png)

* …and not in a dense, small graph
![Time spent on graphs of order 100](doc/report_4_python3_100.png)
![Time spent on graphs of order 100](doc/report_4_java_100.png)
![Time spent on graphs of order 100](doc/report_4_go_100.png)
![Time spent on graphs of order 100](doc/report_4_csharp_100.png)
![Time spent on graphs of order 100](doc/report_4_rust_100.png)


## Introducing parallelism

Let's implement **Ver3-GP** exploiting parallellism (using all CPU cores). How does Ver3 operate?

![Ver3 structure](doc/Ver3.svg)

The first iteration is different from the nested iterations, because:
- the order in which it visits vertices is the degeneracy order, not the GP order;
- the set of candidates starts off being huge, but doesn't have to be represented at all
  because every visited vertex is a candidate until excluded.

Thus we can easily run 2 + N jobs in parallel:
- 1 stage of degeneracy ordering
- 1 stage with the first iteration
- many stages of nested iterations

* **Ver3=GPs:** (C#, Java, Scala) using simple composition (async, stream, future)
* **Ver3=GPc:** (Rust, C++, Java) using something resembling channels
* **Ver3=GP0:** (Go only) using channels and providing 1 goroutine for the recursive calls
* **Ver3=GP1:** (Go only) using channels and providing 4 goroutines for the recursive calls
* **Ver3=GP2:** (Go only) using channels and providing 16 goroutines for the recursive calls
* **Ver3=GPc:** (Go only) using channels and providing 64 goroutines for the recursive calls
* **Ver3=GP4:** (Go only) using channels and providing 256 goroutines for the recursive calls

### Results
* In Java, simpler multi-threading goes a long way, and more elaborate code shaves off a little more
![Time spent on graphs of order 100](doc/report_5_java_100.png)
![Time spent on graphs of order 10k](doc/report_5_java_10k.png)
![Time spent on graphs of order 1M](doc/report_5_java_1M.png)

* In Go, Ver3=GP0 shows the overhead of channels if you don't allow much to operate in parallel;
  and there's no need to severely limit the number of goroutines
![Time spent on graphs of order 100](doc/report_5_go_100.png)
![Time spent on graphs of order 10k](doc/report_5_go_10k.png)
![Time spent on graphs of order 1M](doc/report_5_go_1M.png)

## Comparing languages

* Plain single-threaded
![Time spent on graphs of order 100](doc/report_6_100.png)
![Time spent on graphs of order 10k](doc/report_6_10k.png)
![Time spent on graphs of order 1M](doc/report_6_1M.png)

* Simple multi-threaded
![Time spent on graphs of order 100](doc/report_6_parallel_100.png)
![Time spent on graphs of order 10k](doc/report_6_parallel_10k.png)
![Time spent on graphs of order 1M](doc/report_6_parallel_1M.png)

* Multi-thread using something resembling channels
![Time spent on graphs of order 100](doc/report_6_channels_100.png)
![Time spent on graphs of order 10k](doc/report_6_channels_10k.png)
![Time spent on graphs of order 1M](doc/report_6_channels_1M.png)

## Set data structures

All algorithms work heavily with sets. Some languages allow picking at compile time among
various generic set implementations and compare their performance.

### Rust
* **BTree:** `std::collections::BTreeSet`
* **Hash:** `std::collections::HashSet`, a wrapper around hashbrown
* **hashbrown:** `HashSet` from [crate hashbrown](https://crates.io/crates/hashbrown) 0.11
* **fnv:** `FnvHashSet` from [crate fnv](https://crates.io/crates/fnv) 1.0
* **ord_vec:** ordered `std::collections::Vec` (obviously, this can only work well on small graphs)

### C++
* **std_set:** `std::set`
* **hashset:** `std::unordered_set`
* **ord_vec:** ordered `std::vector` (obviously, this can only work well on small graphs)

### Results

* Rust (multi-threaded use shows very similar results, but less consistent runs)
![Time spent on graphs of order 100](doc/report_7_rust_100.png)
![Time spent on graphs of order 10k](doc/report_7_rust_10k.png)
![Time spent on graphs of order 1M](doc/report_7_rust_1M.png)

In very sparse graphs, only `BTreeSet` allows Ver1 to scale up.

* C++
![Time spent on graphs of order 100](doc/report_7_c++_100.png)
![Time spent on graphs of order 10k](doc/report_7_c++_10k.png)

## How to run & test
Title links to complete benchmark results.

### [Python 3](doc/results_python3.md)

    cd python3
    (once) python -m venv venv
    venv\Scripts\activate.bat
    (once) pip install pytest chart-studio hypothesis mypy
    mypy .
    pytest
    python -O test_maximal_cliques.py

### [Rust](doc/results_rust.md)

    cd rust
    cargo clippy --workspace
    cargo test --workspace
    cargo run --release

### [Go](doc/results_go.md)

    cd go
    go vet BronKerbosch/...
    go test BronKerbosch/...
    go test -race BronKerbosch/lib
    go run BronKerbosch/main
    python ..\python3\publish.py go 100 10k 1M

### [C#](doc/results_csharp.md)
  - open csharp\BronKerboschStudy.sln with Visual Studio 2019
  - set configuration to Debug
  - Test > Run > All Tests
  - set configuration to Release
  - Debug > Start Without Debugging

and finally

    python python3\publish.py c# 100 10k 1M

### [C++ 17](doc/results_cpp.md)
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

### [Java](doc/results_java.md)
  - open folder java with IntelliJ IDEA 2020.2 (Community Edition)
  - set run configuration to "Test"
  - Run > Run 'Test'
  - set run configuration to "Main"
  - Run > Run 'Main'

and finally

    python python3\publish.py java 100 10k 1M

### [Scala](doc/results_scala.md)
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

## License

[BSD License](http://opensource.org/licenses/BSD-3-Clause)
