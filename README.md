[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/ssomers/bron-kerbosch?svg=true&branch=master)](https://ci.appveyor.com/project/ssomers/bron-kerbosch)

## What is this?

Performance comparison of three Bron-Kerbosch algorithm implementations that find all maximal cliques in a graph.
Compared to the original forked from, the code is:
* converted from python 2 to python 3.7
* (hopefully) clarified and type safe
* extended with variations on the algorithms
* extended with unit tests and a performance test on random graphs
* being mirrored in Rust 


## Implementations

* **Ver1:** naive Bron-Kerbosch algorithm
* **Ver2:** Ver1 with pivot
* **Ver3:** Ver2 with degeneracy ordering, picking pivot arbitrarily, and clearly marked as needing a performance fix
* **Ver4:** Ver2 slightly optimized (in vain) and picking pivot randomly (IK\_RP)
* **Ver5:** Ver2 slightly optimized (in vain) and picking pivot smartly (IK\_GPX)
* **Ver6:** Ver3 more optimized (with result, but not enough to beat Ver5)

## Run

    cd python3 && python -O python3/test_maximal_cliques.py
    cd rust && cargo run --release


## Results

Average seconds spent on a particular machine, in particular random graphs (but results seem consistent):

* Dense random graphs of order 50: Ver1 indeed can't cope.
<div><a href="https://plot.ly/~stein.somers/92/?share_key=LVkjsBy9TuBqQLN3COmN9A" target="_blank" title="bron_kerbosch_python3_order_50" style="display: block; text-align: center;"><img src="https://plot.ly/~stein.somers/92.png?share_key=LVkjsBy9TuBqQLN3COmN9A" alt="bron_kerbosch_python3_order_50" style="max-width: 100%;width: 600px;"  width="600" onerror="this.onerror=null;this.src='https://plot.ly/404.png';" /></a></div>
<div><a href="https://plot.ly/~stein.somers/94/?share_key=W9mdBlLlOMJDjj6u1UY9yv" target="_blank" title="bron_kerbosch_rust_order_50" style="display: block; text-align: center;"><img src="https://plot.ly/~stein.somers/94.png?share_key=W9mdBlLlOMJDjj6u1UY9yv" alt="bron_kerbosch_rust_order_50" style="max-width: 100%;width: 600px;"  width="600" onerror="this.onerror=null;this.src='https://plot.ly/404.png';" /></a></div>


* Sparse random graphs of order 10k: Ver3 indeed needed straightening out.
<div><a href="https://plot.ly/~stein.somers/104/?share_key=Me9i7jrXx4fYmWvPWe0oYc" target="_blank" title="bron_kerbosch_python3_order_10k" style="display: block; text-align: center;"><img src="https://plot.ly/~stein.somers/104.png?share_key=Me9i7jrXx4fYmWvPWe0oYc" alt="bron_kerbosch_python3_order_10k" style="max-width: 100%;width: 600px;"  width="600" onerror="this.onerror=null;this.src='https://plot.ly/404.png';" /></a></div>
<div><a href="https://plot.ly/~stein.somers/100/?share_key=i72XBo5uIEvrUe9j7HfL5r" target="_blank" title="bron_kerbosch_rust_order_10k" style="display: block; text-align: center;"><img src="https://plot.ly/~stein.somers/100.png?share_key=i72XBo5uIEvrUe9j7HfL5r" alt="bron_kerbosch_rust_order_10k" style="max-width: 100%;width: 600px;"  width="600" onerror="this.onerror=null;this.src='https://plot.ly/404.png';" /></a></div>


## Test
    
    cd python3 && pytest
    cd rust && cargo test --all


## Time Comlexity

Worst-case time-complexity analysis is [here](http://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm#Worst-case_analysis).

## License

[BSD License](http://opensource.org/licenses/BSD-3-Clause)
