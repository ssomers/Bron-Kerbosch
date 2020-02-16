package main

import (
	"BronKerbosch"
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

func readRandomUndirectedGraph(orderstr string, order int, size int) (BronKerbosch.UndirectedGraph, error) {
	var g BronKerbosch.UndirectedGraph

	fullyMeshedSize := order * (order - 1) / 2
	if size > fullyMeshedSize {
		panic(
			fmt.Sprintf("%d nodes accommodate at most %d edges", order, fullyMeshedSize))
	}

	name := "random_edges_order_" + orderstr + ".txt"
	path := filepath.Join(".", name)
	file, err := os.Open(path)
	if err != nil {
		return g, err
	}
	defer file.Close()
	g.Adjacencies = make([]BronKerbosch.VertexSet, order)
	for v := 0; v < order; v++ {
		g.Adjacencies[v] = make(BronKerbosch.VertexSet)
	}
	scanner := bufio.NewScanner(file)
	linenum := 0
	for scanner.Scan() {
		if linenum == size {
			break
		}
		linenum += 1
		line := scanner.Text()
		fields := strings.SplitN(line, " ", 2)
		v, err := strconv.Atoi(fields[0])
		if err != nil {
			return g, err
		}
		w, err := strconv.Atoi(fields[1])
		if err != nil {
			return g, err
		}
		g.Adjacencies[v].Add(BronKerbosch.Vertex(w))
		g.Adjacencies[w].Add(BronKerbosch.Vertex(v))
	}
	if err := scanner.Err(); err != nil {
		return g, err
	}

	if linenum < size {
		return g, fmt.Errorf("Exhausted generated list of %d edges in %s", linenum, path)
	}
	if g.Order() != order {
		panic("botched order")
	}
	if g.Size() != size {
		panic("botched size")
	}
	return g, nil
}
