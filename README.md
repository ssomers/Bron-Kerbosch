[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/ssomers/bron-kerbosch?svg=true&branch=master)](https://ci.appveyor.com/project/ssomers/bron-kerbosch)

## What is this?

Performance comparison of various implementations of three Bron-Kerbosch algorithms to find all maximal cliques in a graph.
The purpose is not only to compare the algorithms, but also programming lanuages, and the effect of optimization within a programming language.
Compared to the original forked from, the code is:
* converted from python 2 to python 3.7
* (hopefully) clarified and type safe
* extended with variations on the algorithms
* extended with unit tests and a performance test on random graphs
* mirrored in Rust 


## Implementations

* **Ver1:** naive Bron-Kerbosch algorithm
* **Ver2:** Ver1 with pivot, picking pivot arbitrarily
* **Ver3:** Ver2 with degeneracy ordering (clearly marked as needing a performance fix)
* **Ver4:** Ver2 slightly optimized (in vain) and picking pivot randomly (IK\_RP)
* **Ver5:** Ver2 slightly optimized (in vain) and picking pivot smartly (IK\_GPX)
* **Ver6:** Ver3 more optimized (with result, but not enough to beat Ver5)

## Run

    cd python3 && python -O test_maximal_cliques.py
    cd rust && cargo run --release


## Results

Average seconds spent on a particular machine, in particular random graphs (but results seem consistent):

* Dense random graphs of order 50: Ver1 indeed can't cope.

![Python3 graph](https://plot.ly/~stein.somers/126.png?share_key=vE16oDR7OE8KIE909Znmcn)[open](https://plot.ly/~stein.somers/126/?share_key=vE16oDR7OE8KIE909Znmcn)
![Rust graph](https://plot.ly/~stein.somers/122.png?share_key=PwkWG3NLfn7Vg3N6JQi9Pk)[open](https://plot.ly/~stein.somers/122/?share_key=PwkWG3NLfn7Vg3N6JQi9Pk)


* Sparse random graphs of order 10k: Ver3 indeed needed straightening out.

![Python3 graph](https://plot.ly/~stein.somers/128.png?share_key=8AATmcjFpdY0onO7L9nmad)[open](https://plot.ly/~stein.somers/128/?share_key=8AATmcjFpdY0onO7L9nmad)
![Rust graph](https://plot.ly/~stein.somers/124.png?share_key=IFDVpkT7WiFl8n2Cc8Tjnj)[open](https://plot.ly/~stein.somers/124/?share_key=IFDVpkT7WiFl8n2Cc8Tjnj)


## Test
    
    cd python3 && pytest
    cd rust && cargo test --all


## Time Comlexity

Worst-case time-complexity analysis is [here](http://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm#Worst-case_analysis).

## License

[BSD License](http://opensource.org/licenses/BSD-3-Clause)
