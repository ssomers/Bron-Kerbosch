package BronKerbosch

import (
	"fmt"
	"slices"
)

const NumFuncs = 13

var Funcs = [NumFuncs]func(*UndirectedGraph, Consumer){
	bronKerbosch1,
	bronKerbosch2aGP,
	bronKerbosch2bGP,
	bronKerbosch3gp,
	bronKerbosch3gp1,
	bronKerbosch3gp2,
	bronKerbosch3gp3,
	bronKerbosch3gp4,
	bronKerbosch3gp5,
	bronKerbosch3gp6,
	bronKerbosch3gp8,
	bronKerbosch3gp24,
	bronKerbosch3gp72,
}

var FuncNames = [NumFuncs]string{
	"Ver1½",
	"Ver2-GP",
	"Ver2½-GP",
	"Ver3½-GP",
	"Ver3½=GP1",
	"Ver3½=GP2",
	"Ver3½=GP3",
	"Ver3½=GP4",
	"Ver3½=GP5",
	"Ver3½=GP6",
	"Ver3½=GP8",
	"Ver3½=GP24",
	"Ver3½=GP72",
}

// Create a copy and append, resistant to later changes in `head`.
func Append(head Clique, tail Vertex) Clique {
	r := make(Clique, len(head)+1)
	r[copy(r, head)] = tail
	return r
}

func SortCliques(cliques []Clique) {
	for _, clique := range cliques {
		slices.SortFunc(clique, func(l, r Vertex) int {
			return int(l - r)
		})
	}
	slices.SortFunc(cliques, func(l, r Clique) int {
		for i := range min(len(l), len(r)) {
			if d := int(l[i] - r[i]); d != 0 {
				return d
			}
		}
		if len(cliques) < 5 {
			panic(fmt.Sprintf("got overlapping cliques %d <> %d: %v", l, r, cliques))
		}
		panic(fmt.Sprintf("got %d overlapping cliques, e.g. %d <> %d", len(cliques), l, r))
	})
}

func CompareCliques(leftCliques, rightCliques []Clique, errors func(string)) {
	if len(leftCliques) != len(rightCliques) {
		errors(fmt.Sprintf("%d <> %d cliques", len(leftCliques), len(rightCliques)))
	} else {
		for j, l := range leftCliques {
			r := rightCliques[j]
			if !slices.Equal(l, r) {
				errors(fmt.Sprintf("clique %v <> %v", l, r))
			}
		}
	}
}
