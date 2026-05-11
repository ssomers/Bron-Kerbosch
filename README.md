# What is this?

Performance comparison of many implementations to solve one particular computational problem,
to compare the effects of algorithm complexity, programming languages, library choices, and parallelism.

The algorithms implemented are three variants of [the Bron-Kerbosch algorithm](http://en.wikipedia.org/wiki/Bron-Kerbosch_algorithm)
to find maximal cliques in a graph. Some algorithm variants (IK_*) are described in the 2008 paper by F. Cazals & C. Karande,
“A note on the problem of reporting maximal cliques”, Theoretical Computer Science, 407 (1): 564–568, doi:10.1016/j.tcs.2008.05.010.

This project originated as a fork of [cornchz/Bron-Kerbosch](https://github.com/cornchz/Bron-Kerbosch). Compared to the original project, the code is:
* converted from python 2 to python 3;
* (hopefully) clarified and statically typed;
* extended with variations on the algorithms;
* extended with unit tests, property based testing, and this performance test on random graphs;
* done over in Rust, C#, F#, Java, Kotlin, Go, and C++;
* with alternatives exploiting parallelism;
* since March 2026, ignoring maximal 2-cliques, i.e. lone edges, in addition to the original project
  that ignored other trivial maximal cliques: maximal 1-cliques (unconnected vertices)
  and the maximal 0-clique in the empty graph.

All charts below show the amount of time spent on the same particular Windows machine with a 6 core CPU,
all on the same predetermined random graph, with error bars showing the minimum and maximum
over 5 or 3 samples. Order of a graph = number of vertices, size of a graph = number of edges.

A random graph is easy to generate and objective, but not ideal to test the performance of the
algorithm itself, because when you're doing something useful looking for maximal cliques, the actual data likely
comes in cliques, some of which are near-maximal and cause the heartaches described in the paper.


# Executive summary
* Better algorithms, invented to counter treacherous cases, stand their ground on a vanilla random graph.
* Programming language makes a difference, as in a factor of 2 up to 8.
  - Rust is clearly the fastest, but beware I contributed several performance improvements to its
    collection library, more than I invested in optimally using the collection libraries of
    the other, more established languages.
  - C# is the runner up, surpringly (to me).
  - Python is the slowest, not surprisingly.
  - C++ is clearly not the fastest (and I claim this with the confidence of 20 years of professional C++ development).
* Multi-threading helps a lot too, and how programming languages accommodate for it makes a huge difference.
  Python is the worst in that respect, I couldn't get any multi-threading code to work faster than the single-threaded code.
* Collection libraries don't matter much, though hashing-based collection reach sizes that a B-tree can only dream of.


# Detailed reports
## Local optimization

Let's first get one thing out of the way: what does some local optimization yield in the simplest,
naive Bron-Kerbosch algorithm, in Python and Rust. Is this premature optimization or low hanging fruit?

* **Ver1:** Same as in the original project
* **Ver1½:** Same locally optimized, without changing the algorithm as such.
In particular:
  - In the (many) deepest iterations, when we see the intersection of candidates is empty, don't
    calculate all the nearby excluded vertices, just check if that set is empty or not.
  - We implement intersection with performance in mind, unlike what the standard libraries of most
    programming languages offer.
  - In Rust, compile a `Clique` from the call stack, instead of passing it around on the heap.
    Basically showing off Rust's ability to guarantee, at compile time, this can be done safely.

### Results

We gain almost as much as through switching to the best performing programming language

![Time spent on graphs of order 100](doc/report_1.svg)

Therefore, all the other implementations will contain similar tweaks.

## Comparing algorithms

* **Ver2:** Ver1 excluding neighbours of a pivot that is chosen arbitrarily
* **Ver2-GP:** Ver2 but pivot is the candidate of the highest degree towards the remaining candidates (IK\_GP in the paper)
* **Ver2-GPX:** Ver2-GP but pivot also chosen from excluded vertices (IK\_GPX in the paper)
* **Ver2-RP:** Similar but but with pivot randomly chosen from candidates (IK\_RP in the paper)
* **Ver3:** Ver2 with degeneracy ordering
* **Ver3-GP:** Ver2-GP with degeneracy ordering
* **Ver3-GPX:** Ver2-GPX with degeneracy ordering

We mostly implement locally optimized **½** versions of these.
In addition to the local optimizations mentioned above:
- We [write out the first iteration separately](https://github.com/ssomers/Bron-Kerbosch/blob/master/python3/bron_kerbosch2_gp.py),
  because in that first iteration the set of candidate vertices starts off being huge,
  with every connected vertex in the graph, but that set doesn't have to be represented at all
  because every reachable vertex is a candidate until excluded.
- In the same first iteration, we store the set of excluded vertices as an array of booleans,
  because as a regular set it ends up being huge, and most of the time the set we intersect with
  (the set of neighbours of some vertex) will be smaller, so all that matters is lookup speed.
- Or even better, in Ver3 algorithms, the degeneracy calculation already provides the set of excluded neighbours. All the first iteration needs to do is extract that information.

These are all single-threaded implementations (using only one CPU core).

### Results

* Ver1 indeed struggles with dense graphs, when it has to cover more than half of the 4950 possible edges

![Time spent on graphs of order 100](doc/report_2.svg)

* Among Ver2 variants, GP and GPX are indeed best…

![Time spent on graphs of order 100](doc/report_3_java_100.svg)
![Time spent on graphs of order 100](doc/report_3_rust_100.svg)

* …but GPX looses ground in big, sparse graphs

![Time spent on graphs of order 10k](doc/report_3_java_10k.svg)
![Time spent on graphs of order 10k](doc/report_3_rust_10k.svg)

* Ver3-GP barely wins from Ver2-GP in moderately sized graphs…

![Time spent on graphs of order 10k](doc/report_4_rust_10k.svg)
![Time spent on graphs of order 10k](doc/report_4_csharp_10k.svg)

* …but loses in many other cases

![Time spent on graphs of order 1M](doc/report_4_csharp_1M.svg)
![Time spent on graphs of order 10k](doc/report_4_java_10k.svg)
![Time spent on graphs of order 10k](doc/report_4_python314_10k.svg)
![Time spent on graphs of order 1M](doc/report_4_python314_1M.svg)

* Ver3-GP seems to cope slightly better at scale than Ver3-GPX

![Time spent on graphs of order 1M](doc/report_5_csharp_10k.svg)
![Time spent on graphs of order 1M](doc/report_5_csharp_1M.svg)
![Time spent on graphs of order 10k](doc/report_5_java_10k.svg)
![Time spent on graphs of order 10k](doc/report_5_rust_10k.svg)
![Time spent on graphs of order 10k](doc/report_5_python314_10k.svg)
![Time spent on graphs of order 1M](doc/report_5_python314_1M.svg)

## Introducing parallelism

Let's implement **Ver3-GP** exploiting parallellism (using all CPU cores). How does Ver3 operate?

```mermaid
flowchart TD
  subgraph s[ ]
    a[degeneracy order]
    b[first iteration]
    a --> b
  end
  c[nested iteration]
  d[nested iteration]
  e[nested iteration]
  f[nested iteration]
  g[nested iteration]
  h[nested iteration]
  b --> c
  b --> d
  c --> e
  c --> f
  c --> g
  c --> h
  d --> e
  d --> f
  d --> g
  d --> h
```

We have:
- 1 task generating the degeneracy order of the graph.
- 1 task performing the first iteration in that order; however, since it relies heavily on data supporting the degeneracy order, it cannot be run in parallel.
- N tasks each performing nested iterations.

So we bundle the first two tasks and run 1 + N tasks in parallel.

Ways to implement parallelism varies per language:
* **Ver3½=GPc:** (Rust, Java, Kotlin) using one thread per task as described above, somewhat complicated
* **Ver3½=GPs:** (C#, Java, Kotlin) using relatively simple composition backed by synchronized containers
* **Ver3½=GP0:** (Go) using channels and 1 goroutine handling the nested iterations
* **Ver3½=GP1:** (Go) using channels and 4 goroutines handling the nested iterations
* **Ver3½=GP2:** (Go) using channels and 16 goroutines handling the nested iterations
* **Ver3½=GP3:** (Go) using channels and 64 goroutines handling the nested iterations
* **Ver3½=GP4:** (Go) using channels and 256 goroutines handling the nested iterations

### Results
* In Java, simpler multi-threading goes a long way, and more elaborate code shaves off a little more

![Time spent on graphs of order 100](doc/report_6_java_100.svg)
![Time spent on graphs of order 10k](doc/report_6_java_10k.svg)
![Time spent on graphs of order 1M](doc/report_6_java_1M.svg)

* In Go, C#, Kotlin and Rust, the CPU clearly has 6 cores willing to work. There's no use in tuning the number of goroutines in Go or MaxDegreeOfParallelism in C#, while in Rust, too many explicit threads incur a slight cost.

![Time spent on graphs of order 100](doc/report_6_go_100.svg)
![Time spent on graphs of order 10k](doc/report_6_go_10k.svg)
![Time spent on graphs of order 1M](doc/report_6_go_1M.svg)

![Time spent on graphs of order 100](doc/report_6_csharp_100.svg)
![Time spent on graphs of order 10k](doc/report_6_csharp_10k.svg)
![Time spent on graphs of order 1M](doc/report_6_csharp_1M.svg)

![Time spent on graphs of order 100](doc/report_6_kotlin_100.svg)
![Time spent on graphs of order 10k](doc/report_6_kotlin_10k.svg)
![Time spent on graphs of order 1M](doc/report_6_kotlin_1M.svg)

![Time spent on graphs of order 100](doc/report_6_rust_100.svg)
![Time spent on graphs of order 10k](doc/report_6_rust_10k.svg)
![Time spent on graphs of order 1M](doc/report_6_rust_1M.svg)


## Comparing languages

* Plain single-threaded

![Time spent on graphs of order 100](doc/report_7_sequential_100.svg)
![Time spent on graphs of order 10k](doc/report_7_sequential_10k.svg)
![Time spent on graphs of order 1M](doc/report_7_sequential_1M.svg)

* Multi-threaded

![Time spent on graphs of order 100](doc/report_7_parallel.svg)
![Time spent on graphs of order 10k](doc/report_7_parallel.svg)
![Time spent on graphs of order 1M](doc/report_7_parallel.svg)


## Comparing versions of languages

* Python 3.10 (before a performance boost) versus 3.14 (latest)

![Time spent on graphs of order 100](doc/report_9_python_100.svg)
![Time spent on graphs of order 10k](doc/report_9_python_10k.svg)
![Time spent on graphs of order 1M](doc/report_9_python_1M.svg)

## Comparing implementations of the set data structure

All algorithms work heavily with sets. Some languages allow picking at compile time among
various generic set implementations.

### Rust
* **BTree:** `std::collections::BTreeSet`
* **Hash:** `std::collections::HashSet`, a wrapper around a version of hashbrown
* **hashbrown:** `HashSet` from [crate hashbrown](https://crates.io/crates/hashbrown) 0.16
* **fnv:** `FnvHashSet` from [crate fnv](https://crates.io/crates/fnv) 1.0.7
* **ord_vec:** ordered `std::collections::Vec`

#### Results

* Rust (multi-threaded use shows very similar results, but less consistent runs)

![Time spent on graphs of order 100](doc/report_8_rust_100.svg)
![Time spent on graphs of order 10k](doc/report_8_rust_10k.svg)
![Time spent on graphs of order 1M](doc/report_8_rust_1M.svg)

In very sparse graphs, only `BTreeSet` allows Ver1 to scale up.

### C++
* **std_set:** `std::set`
* **hashset:** `std::unordered_set`
* **ord_vec:** ordered `std::vector` (obviously, this can only work well on small graphs)

#### Results

![Time spent on graphs of order 100](doc/report_8_cpp_100.svg)
![Time spent on graphs of order 10k](doc/report_8_cpp_10k.svg)


### C#
* **HashSet**
* **SortedSet:**

#### Results

![Time spent on graphs of order 100](doc/report_8_csharp_100.svg)
![Time spent on graphs of order 10k](doc/report_8_csharp_10k.svg)


# How to run & test

## Python 3

    cd python3
    (once) python -m venv venv
    venv\Scripts\activate.bat
    (once or twice) pip install --upgrade black mypy ruff pytest hypothesis matplotlib
    (if edited) black .
    (if edited) ruff check . --exclude "venv*"
    (if edited) mypy .
    (if edited) pytest
    python -O main.py

## Rust

    cd rust
    (once) cargo install cargo-edit
    (sometimes) rustup update
    (sometimes) cargo upgrade && cargo update
    (if edited) cargo fmt --all
    (if edited) cargo clippy --workspace --tests
    (if edited) cargo test --workspace
    cargo run --release

## Go

    cd go
    (if edited) go fmt ./...
    (if edited) go vet ./...
    (if edited) revive -config revive.toml -formatter friendly ./...
    (if edited) go test ./...
    (if edited) go test ./Stats -fuzz=Stats1 -fuzztime=1s
    (if edited) go test ./Stats -fuzz=Stats2 -fuzztime=2s
    (if edited) go test ./Stats -fuzz=StatsN -fuzztime=5s
    (if edited) go test ./BronKerbosch -fuzz=DegeneracyOrder -fuzztime=20s
    go run main.go

Optionally, on MSYS2:

    PATH=$PATH:$PROGRAMFILES/go/bin
    go test -race ./BronKerbosch


## C#

  - open csharp\BronKerboschStudy.sln with Visual Studio 2026
  - (if edited) set configuration to Debug
  - (if edited) Test > Run > All Tests
  - set configuration to Release
  - Debug > Start Without Debugging


## F#

  - open fsharp\BronKerbosch.slnx with Visual Studio 2026
  - (if edited) set configuration to Debug
  - (if edited) Test > Run > All Tests
  - set configuration to Release
  - Debug > Start Without Debugging


## C++ 20

  - open cpp\BronKerboschStudy.sln with Visual Studio 2022
  - (if edited) set configuration to Debug
  - (if edited) Test > Run > All Tests
  - set configuration to Release
  - Debug > Start Without Debugging


## Java

  - open folder java with IntelliJ IDEA 2026 (Community Edition)
  - (if edited) set run configuration to "Test"
  - (if edited) Run > Run 'Test'
  - set run configuration to "Main"
  - Run > Run 'Main'


## Kotlin

  - open folder kotlin with IntelliJ IDEA 2026 (Community Edition)
  - (if edited) set run configuration to "Test"
  - (if edited) Run > Run 'Test'
  - set run configuration to "Main"
  - Run > Run 'Main'


## Finally

And finally, generate reports:

    python ..\python3\publish.py

## License

[BSD License](http://opensource.org/licenses/BSD-3-Clause)
