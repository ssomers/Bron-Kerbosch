[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/ssomers/bron-kerbosch?svg=true&branch=master)](https://ci.appveyor.com/project/ssomers/bron-kerbosch)

## What is this?

Performance comparison of three Bron-Kerbosch algorithm implementations that find all maximal cliques in a graph.
Compared to the original forked from, the code is:
* converted from python 2 to python 3.7
* (hopefully) clarified and type safe
* extended with variations on the algorithms
* extended with unit tests and a performance test on random graphs


## Implementations

* **Ver1:** naive Bron-Kerbosch algorithm
* **Ver2:** Ver1 with pivot
* **Ver3:** Ver2 with degeneracy ordering, picking pivot arbitrarily
* **Ver4:** Ver2 slightly optimized (in vain) and picking pivot randomly (IK\_RP)
* **Ver5:** Ver2 slightly optimized (in vain) and picking pivot smartly (IK\_GPX)
* **Ver6:** Ver3 more optimized (with result)

## Run

    python -O test_maximal_cliques.py


## Results

Average seconds spent on a particular machine, in particular random graphs (but results seem consistent):

* Dense random graphs of order 50: Ver1 sucks
<div><a href="https://plot.ly/~stein.somers/64/?share_key=ddjMPag8Q6y561Ozvjm7cR" target="_blank" title="Bron-Kerbosch on order 50" style="display: block"><img src="https://plot.ly/~stein.somers/64.png?share_key=ddjMPag8Q6y561Ozvjm7cR" alt="Bron-Kerbosch on order 50" style="max-width: 100%;width: 600px;"  width="600" onerror="this.onerror=null;this.src='https://plot.ly/404.png';" /></a><script data-plotly="stein.somers:64" sharekey-plotly="ddjMPag8Q6y561Ozvjm7cR" src="https://plot.ly/embed.js" async></script></div>

* Spare random graphs of order 10k:
<div><a href="https://plot.ly/~stein.somers/66/?share_key=AncArWLi5zvOcwr7e3laTj" target="_blank" title="Bron-Kerbosch on order 10k" style="display: block"><img src="https://plot.ly/~stein.somers/66.png?share_key=AncArWLi5zvOcwr7e3laTj" alt="Bron-Kerbosch on order 10k" style="max-width: 100%;width: 600px;"  width="600" onerror="this.onerror=null;this.src='https://plot.ly/404.png';" /></a><script data-plotly="stein.somers:66" sharekey-plotly="AncArWLi5zvOcwr7e3laTj" src="https://plot.ly/embed.js" async></script></div>


## Test
    
    pytest


## Time Comlexity

Worst-case time-complexity analysis is [here](http://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm#Worst-case_analysis).

## License

[BSD License](http://opensource.org/licenses/BSD-3-Clause)
