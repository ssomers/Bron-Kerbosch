package BronKerbosch

import (
	"fmt"
	"sort"
	"time"
)

const numFuncs = 10

var funcs = [numFuncs]func(*UndirectedGraph) [][]Vertex{
	bronKerbosch1,
	bronKerbosch2g, bronKerbosch2gp, bronKerbosch2gpx,
	bronKerbosch3gp, bronKerbosch3gpx,
	bronKerbosch3gp2,
	bronKerbosch3gp3,
	bronKerbosch3gp4,
	bronKerbosch3gp5,
}

var FuncNames = [numFuncs]string{
	"Ver1+",
	"Ver2+G", "Ver2+GP", "Ver2+GPX",
	"Ver3+GP", "Ver3+GPX",
	"Ver3+GP2",
	"Ver3+GP3",
	"Ver3+GP4",
	"Ver3+GP5",
}

func sortCliques(cliques [][]Vertex) {
	for _, clique := range cliques {
		sort.Slice(clique, func(l int, r int) bool {
			return clique[l] < clique[r]
		})
	}
	sort.Slice(cliques, func(l int, r int) bool {
		for i := 0; i < len(cliques[l]) && i < len(cliques[r]); i++ {
			if d := cliques[l][i] - cliques[r][i]; d != 0 {
				return d < 0
			}
		}
		if len(cliques) < 10 {
			panic(fmt.Sprintf("got overlapping cliques %d <> %d: %v", l, r, cliques))
		} else {
			panic(fmt.Sprintf("got overlapping cliques: #%d %d <> #%d %d",
				l+1, cliques[l],
				r+1, cliques[r]))
		}
	})
}

func compareCliques(leftCliques [][]Vertex, rightCliques [][]Vertex, errors func(string)) {
	if len(leftCliques) != len(rightCliques) {
		errors(fmt.Sprintf("%d <> %d cliques", len(leftCliques), len(rightCliques)))
	} else {
		for j, left := range leftCliques {
			right := rightCliques[j]
			if len(left) != len(right) {
				errors(fmt.Sprintf("clique #%d: %d <> %d vertices", j+1, len(left), len(right)))
			} else {
				for i, l := range left {
					r := right[i]
					if l != r {
						errors(fmt.Sprintf("clique #%d vertex #%d/%d: %d <> %d", j+1, i+1, len(left), l, r))
					}
				}
			}
		}
	}
}

func Timed(order int, size int, samples int) [numFuncs]SampleStatistics {
	var times [numFuncs]SampleStatistics
	graph := randomUndirectedGraph(order, size)
	var first [][]Vertex
	for sample := 0; sample < samples; sample++ {
		for funcIndex, bronKerboschFunc := range funcs {
			begin := time.Now()
			current := bronKerboschFunc(&graph)
			secs := time.Since(begin).Seconds()
			if secs >= 3.0 {
				fmt.Printf("  %-8s: %5.2fs\n", FuncNames[funcIndex], secs)
			}
			if sample < 2 {
				sortCliques(current)
				if len(first) == 0 {
					first = current
				} else {
					compareCliques(current, first, func(e string) {
						fmt.Printf("  %s: %s\n", FuncNames[funcIndex], e)
					})
				}
			}
			times[funcIndex].Put(secs)
		}
	}
	return times
}
