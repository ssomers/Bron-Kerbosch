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

Seconds spent on a particular machine, in particular random graphs (but results seem consistent), averaged over 7 runs:

* random graph of order 26 (full mesh size 325):
<table>
<tr><th>size</th><th>Ver1</th><th>Ver2</th><th>Ver3</th><th>Ver4</th><th>Ver5</th><th>Ver6</th>
<tr><th>220</th><td>0.01</td><td>0.00</td><td>0.00</td><td>0.01</td><td>0.00</td><td>0.01</td></tr>
<tr><th>240</th><td>0.04</td><td>0.01</td><td>0.00</td><td>0.01</td><td>0.00</td><td>0.01</td></tr>
<tr><th>260</th><td>0.09</td><td>0.01</td><td>0.01</td><td>0.01</td><td>0.00</td><td>0.01</td></tr>
<tr><th>280</th><td>0.46</td><td>0.00</td><td>0.01</td><td>0.01</td><td>0.00</td><td>0.01</td></tr>
<tr><th>300</th><td>2.62</td><td>0.01</td><td>0.00</td><td>0.01</td><td>0.00</td><td>0.01</td></tr>
</table>

* random graph of order 82 (full mesh size 3321):
<table>
<tr><th>size</th><th>Ver1</th><th>Ver2</th><th>Ver3</th><th>Ver4</th><th>Ver5</th><th>Ver6</th>
<tr><th>1800</th><td>0.42</td><td>0.23</td><td>0.20</td><td>0.65</td><td>0.14</td><td>0.65</td></tr>
<tr><th>2000</th><td>2.05</td><td>0.73</td><td>0.60</td><td>1.66</td><td>0.32</td><td> 1.70</td></tr>
<tr><th>2200</th><td>7.19</td><td>2.36</td><td>2.05</td><td>4.80</td><td>0.87</td><td> 4.75</td></tr>
</table>

* random graph of order 10k:
<table>
<tr><th>size</th><th>Ver1</th><th>Ver2</th><th>Ver3</th><th>Ver4</th><th>Ver5</th><th>Ver6</th>
<tr><th>10k</th><td>0.05</td><td>0.07</td><td>3.78</td><td>0.15</td><td>0.08</td><td>0.18</td></tr>
<tr><th>20k</th><td>0.09</td><td>0.11</td><td>4.87</td><td>0.20</td><td>0.15</td><td>0.25</td></tr>
<tr><th>30k</th><td>0.13</td><td>0.16</td><td>5.23</td><td>0.25</td><td>0.21</td><td>0.31</td></tr>
<tr><th>40k</th><td>0.17</td><td>0.21</td><td>5.15</td><td>0.30</td><td>0.27</td><td>0.36</td></tr>
<tr><th>50k</th><td>0.22</td><td>0.25</td><td>5.24</td><td>0.36</td><td>0.34</td><td>0.42</td></tr>
<tr><th>60k</th><td>0.26</td><td>0.31</td><td>5.26</td><td>0.42</td><td>0.40</td><td>0.48</td></tr>
</table>


## Test
    
    pytest


## Time Comlexity

Worst-case time-complexity analysis is [here](http://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm#Worst-case_analysis).

## License

[BSD License](http://opensource.org/licenses/BSD-3-Clause)
