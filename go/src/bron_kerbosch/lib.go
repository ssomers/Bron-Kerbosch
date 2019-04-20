package bron_kerbosch

import (
	"fmt"
	"sort"
	"time"
)

const NUM_FUNCS = 7

var FUNCS = [NUM_FUNCS]func(*UndirectedGraph) [][]Vertex{
	bron_kerbosch1,
	bron_kerbosch2_g, bron_kerbosch2_gp, bron_kerbosch2_gpx,
	bron_kerbosch3_gp, bron_kerbosch3_gpx, bron_kerbosch3om}

var FUNC_NAMES = [NUM_FUNCS]string{
	"Ver1+",
	"Ver2+G", "Ver2+GP", "Ver2+GPX",
	"Ver3+GP", "Ver3+GPX", "Ver3+MT"}

func sort_cliques(cliques [][]Vertex) {
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

func compare_cliques(left_cliques [][]Vertex, right_cliques [][]Vertex, errors func(string)) {
	if len(left_cliques) != len(right_cliques) {
		errors(fmt.Sprintf("%d <> %d cliques", len(left_cliques), len(right_cliques)))
	} else {
		for j, left := range left_cliques {
			right := right_cliques[j]
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

func Timed(order int, size int, samples int) [NUM_FUNCS]SampleStatistics {
	var times [NUM_FUNCS]SampleStatistics
	graph := random_undirected_graph(order, size)
	var first [][]Vertex
	for sample := 0; sample < samples; sample++ {
		for func_index, bron_kerbosch_func := range FUNCS {
			begin := time.Now()
			current := bron_kerbosch_func(&graph)
			secs := time.Since(begin).Seconds()
			if secs >= 3.0 {
				fmt.Printf("  %-8s: %5.2fs\n", FUNC_NAMES[func_index], secs)
			}
			if sample < 2 {
				sort_cliques(current)
				if len(first) == 0 {
					first = current
				} else {
					compare_cliques(current, first, func(e string) {
						fmt.Printf("  %s: %s\n", FUNC_NAMES[func_index], e)
					})
				}
			}
			times[func_index].Put(secs)
		}
	}
	return times
}
