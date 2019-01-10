[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/ssomers/bron-kerbosch?svg=true&branch=master)](https://ci.appveyor.com/project/ssomers/bron-kerbosch)

## What is this?

Performance comparison of various implementations of three Bron-Kerbosch algorithms to find all maximal cliques in a graph.
The purpose is not only to compare the algorithms, but also programming lanuages, and the effect of optimization within a programming language.
Compared to the original forked from, the code is:
* converted from python 2 to python 3.7
* (hopefully) clarified and type safe
* extended with variations on the algorithms
* extended with unit tests and a performance test on random graphs
* all that mirrored in Rust 


## Implementations

* **Ver1:** Naive Bron-Kerbosch algorithm
* **Ver2:** Ver1 with pivot, picking pivot arbitrarily
* **Ver3:** Ver2 with degeneracy ordering (clearly marked as needing a performance fix)
* **Ver1+:** Ver1 slightly optimized with language-specific tweaks
* **Ver2_RP:** Ver2 slightly optimized and picking pivot randomly (IK\_RP)
* **Ver2_GP:** Ver2 slightly optimized and picking pivot with highest degree (IK\_GP)
* **Ver2_GPX:** Ver2 slightly optimized and picking pivot with highest degree towards the remaining candidates (IK\_GPX)
* **Ver3+:** Ver3 optimized with scalability and language-specific tweaks (with result, but not enough to beat those without degeneracy ordering, for these graphs)
* **Ver3+MT:** (Rust only) Ver3+ with multi-threading (4 threads, measured on a CPU with 2 cores and hyperthreading)
* **Ver3-:** (Python only) Ver3 with simplified order, determined by degree only

## Run

    cd python3 && python -O test_maximal_cliques.py
    cd rust && cargo run --release


## Results

Average seconds spent on a particular machine, in particular random graphs (but results seem consistent accross the random seed):

* Dense random graphs of order 100: Ver1 indeed can't cope.

[![Time spent in Python3 on order 100 graphs](https://plot.ly/~stein.somers/157.png?share_key=FMnLKjdaEhpyZlGG6nH09O "View interactively")](https://plot.ly/~stein.somers/157/?share_key=FMnLKjdaEhpyZlGG6nH09O)
[![Time spent in Rust on order 100 graphs](https://plot.ly/~stein.somers/153.png?share_key=AvQmqLCv53BIi1Hj30a8Dd "View interactively")](https://plot.ly/~stein.somers/153/?share_key=AvQmqLCv53BIi1Hj30a8Dd)


* Sparse random graphs of order 10k: Ver3 indeed needed straightening out.

[![Time spent in Python3 on order 10k graphs](https://plot.ly/~stein.somers/128.png?share_key=8AATmcjFpdY0onO7L9nmad "View interactively")](https://plot.ly/~stein.somers/128/?share_key=8AATmcjFpdY0onO7L9nmad)
[![Time spent in Rust on order 10k graphs](https://plot.ly/~stein.somers/124.png?share_key=IFDVpkT7WiFl8n2Cc8Tjnj "View interactively")](https://plot.ly/~stein.somers/124/?share_key=IFDVpkT7WiFl8n2Cc8Tjnj)

* Sparse random graphs of order 1M: who scales best?
[![Time spent in Rust on order 1M graphs](https://plot.ly/~stein.somers/155.png?share_key=n5CnokKbHg9fwfBXyyUMOU "View interactively")](https://plot.ly/~stein.somers/155/?share_key=n5CnokKbHg9fwfBXyyUMOU)

## Test
    
    cd python3 && pytest
    cd rust && cargo test --all


## Context

[More information on Wikipedia](http://en.wikipedia.org/wiki/Bron-Kerbosch_algorith).

Most algorithms variants are described in the 2008 paper by F. Cazals & C. Karande, “A note on the problem of reporting maximal cliques”, Theoretical Computer Science, 407 (1): 564–568, doi:10.1016/j.tcs.2008.05.010.

## License

[BSD License](http://opensource.org/licenses/BSD-3-Clause)
