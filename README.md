[![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/github/ssomers/bron-kerbosch?svg=true&branch=master)](https://ci.appveyor.com/project/ssomers/bron-kerbosch)

## What is this?

Performance comparison of various implementations of three Bron-Kerbosch algorithms to find all maximal cliques in a graph.
The purpose is not only to compare the algorithms, but also programming lanuages, and the effect of optimization within a programming language.
Compared to the original forked from, the code is:
* converted from python 2 to python 3.7
* (hopefully) clarified and type safe
* extended with variations on the algorithms
* extended with unit tests and a performance test on random graphs
* all that mirrored in Rust, Go, and partly in C# and Scala

## Implementations

* **Ver1:** Naive Bron-Kerbosch algorithm
* **Ver1+:** Ver1 optimized, including language-specific tweaks
* **Ver2+:** Ver1+ excluding neighbours of a pivot that is chosen arbitrarily
* **Ver2+G:** Similar but with pivot of highest degree in the whole graph, chosen from candidates only
* **Ver2+GP:** Similar but with pivot of highest degree within the remaining candidates, chosen from candidates only (IK\_GP)
* **Ver2+GPX:** Similar but with pivot of highest degree within the remaining candidates, chosen from both candidates and excluded (IK\_GPX)
* **Ver2+RP:** Similar but but with pivot randomly chosen from candidates (IK\_RP)
* **Ver3+:** Ver2+ with degeneracy ordering (optimized, where the original clearly marked it necessary)
* **Ver3+GP:** Ver2+GP with degeneracy ordering
* **Ver3+GPX:** Ver2+GPX with degeneracy ordering
* **Ver3+MT:** (Rust only) Ver3+GP with multi-threading (2 + 5 threads on a 6 core CPU)
* **Ver3+GP2:** (Go only) Ver3+GP with multi-threading (2 + 5 goroutines a 6 core CPU)
* **Ver3+GP3:** (Go only) Ver3+GP with multi-threading (2 + 15 goroutines a 6 core CPU)
* **Ver3+GP4:** (Go only) Ver3+GP with multi-threading (2 + 45 goroutines a 6 core CPU)
* **Ver3+GP5:** (Go only) Ver3+GP with multi-threading (2 + 135 goroutines a 6 core CPU)

## Results

Average seconds spent on a particular machine, in particular random graphs (but results seem consistent accross the random seed):

* Dense random graphs of order 100: Ver1 indeed can't cope.

[![Time spent in Rust on graphs of order 100](https://plot.ly/~stein.somers/153.png?share_key=AvQmqLCv53BIi1Hj30a8Dd "View interactively")](https://plot.ly/~stein.somers/153/?share_key=AvQmqLCv53BIi1Hj30a8Dd)
[![Time spent in C# on graphs of order 100](https://plot.ly/~stein.somers/237.png?share_key=DqinsfUmGJVNhW9jPg5r4S "View interactively")](https://plot.ly/~stein.somers/237/?share_key=DqinsfUmGJVNhW9jPg5r4S)
[![Time spent in Go on graphs of order 100](https://plot.ly/~stein.somers/183.png?share_key=WseMiu6UJZgAKyQvTF2bJp "View interactively")](https://plot.ly/~stein.somers/183/?share_key=WseMiu6UJZgAKyQvTF2bJp)
[![Time spent in Scala on graphs of order 100](https://plot.ly/~stein.somers/197.png?share_key=F33K579eKLzY0A7fSherUI "View interactively")](https://plot.ly/~stein.somers/197/?share_key=F33K579eKLzY0A7fSherUI)
[![Time spent in Python3 on graphs of order 100](https://plot.ly/~stein.somers/157.png?share_key=FMnLKjdaEhpyZlGG6nH09O "View interactively")](https://plot.ly/~stein.somers/157/?share_key=FMnLKjdaEhpyZlGG6nH09O)

* Random graphs of order 10k:

[![Time spent in Rust on graphs of order 10k](https://plot.ly/~stein.somers/124.png?share_key=IFDVpkT7WiFl8n2Cc8Tjnj "View interactively")](https://plot.ly/~stein.somers/124/?share_key=IFDVpkT7WiFl8n2Cc8Tjnj)
[![Time spent in C# on graphs of order 10k](https://plot.ly/~stein.somers/239.png?share_key=SAxyM6pj8iMenK6SfIr9Um "View interactively")](https://plot.ly/~stein.somers/239/?share_key=SAxyM6pj8iMenK6SfIr9Um)
[![Time spent in Go on graphs of order 10k](https://plot.ly/~stein.somers/187.png?share_key=EtNe8FbmD8BwrxBaC7dHBt "View interactively")](https://plot.ly/~stein.somers/187/?share_key=EtNe8FbmD8BwrxBaC7dHBt)
[![Time spent in Scala on graphs of order 10k](https://plot.ly/~stein.somers/199.png?share_key=ZM9Igh4glwfW0rFVJFzf3s "View interactively")](https://plot.ly/~stein.somers/199/?share_key=ZM9Igh4glwfW0rFVJFzf3s)
[![Time spent in Python3 on graphs of order 10k](https://plot.ly/~stein.somers/128.png?share_key=8AATmcjFpdY0onO7L9nmad "View interactively")](https://plot.ly/~stein.somers/128/?share_key=8AATmcjFpdY0onO7L9nmad)

* Very sparse large graphs: Ver1 doesn't scale in some implementations

[![Time spent in Rust on graphs of order 1M](https://plot.ly/~stein.somers/265.png?share_key=pr6lBghz9gi7b16rFrFS6n "View interactively")](https://plot.ly/~stein.somers/265/?share_key=pr6lBghz9gi7b16rFrFS6n)
[![Time spent in C# on graphs of order 1M](https://plot.ly/~stein.somers/267.png?share_key=Q6X4A2vOdBvCKcsEkVKoI3 "View interactively")](https://plot.ly/~stein.somers/267/?share_key=Q6X4A2vOdBvCKcsEkVKoI3)

* Random graphs of order 1M: who scales best?

[![Time spent in Rust on graphs of order 1M](https://plot.ly/~stein.somers/155.png?share_key=n5CnokKbHg9fwfBXyyUMOU "View interactively")](https://plot.ly/~stein.somers/155/?share_key=n5CnokKbHg9fwfBXyyUMOU)
[![Time spent in C# on graphs of order 1M](https://plot.ly/~stein.somers/261.png?share_key=DSsVRnQJEqbKPxQn8RyaSX "View interactively")](https://plot.ly/~stein.somers/261/?share_key=DSsVRnQJEqbKPxQn8RyaSX)
[![Time spent in Go on graphs of order 1M](https://plot.ly/~stein.somers/189.png?share_key=55O2tqyLcqoFVfH89tWesI "View interactively")](https://plot.ly/~stein.somers/189/?share_key=55O2tqyLcqoFVfH89tWesI)
[![Time spent in Scala on graphs of order 1M](https://plot.ly/~stein.somers/201.png?share_key=pAZbwXAIC0C96nD9WP38yl "View interactively")](https://plot.ly/~stein.somers/201/?share_key=pAZbwXAIC0C96nD9WP38yl)
[![Time spent in Python3 on graphs of order 1M](https://plot.ly/~stein.somers/213.png?share_key=FNQg1eSkoQaxjuw5yoEwNJ "View interactively")](https://plot.ly/~stein.somers/213/?share_key=FNQg1eSkoQaxjuw5yoEwNJ")

## Run & Test

### Run Python 3

    cd python3
    (once) python -m venv venv
    venv\Scripts\activate.bat
    (once) pip install pytest plotly
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
    go vet bron_kerbosch main
    go test -race bron_kerbosch

### Run C#
  - open csharp\BronKerboschStudy.sln with Visual Studio Community 2017
  - set configuration to Release
  - Debug > Start Without Debuggging

and finally

    python python3\publish.py c# 100 10k 999k 1M

### Test C#
  - set configuration to Debug
  - Test > Run > All Tests

### Run Scala
  - open scala\bron-kerbosch.iml with IntelliJ IDEA 2019.1.1 (Community Edition)
  - set compiler configuration to release: open File > Settings > Build, Execution, Deployment > Compiler > Scala Compiler; select bron-kerbosch and move to release profile; change something else so IntelliJ doesn't ignore you, Apply (upon which IntelliJ applies the profile change and sometimes the something else), revert the something else and Apply (all this just to compile with -Xdisable-assertions)
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
