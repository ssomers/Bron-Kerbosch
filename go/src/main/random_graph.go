package main

import (
	"BronKerbosch"
	"bufio"
	"fmt"
	"os"
	"path/filepath"
)

func parsePositiveInt(str string) (int, error) {
	var val int
	var suffix rune
	n, err := fmt.Sscanf(str, "%d%c", &val, &suffix)
	if n == 1 {
		return val, nil // ignore EOF error
	}
	if err == nil {
		if suffix == 'k' {
			return val * 1e3, nil
		}
		if suffix == 'M' {
			return val * 1e6, nil
		}
		err = fmt.Errorf("Unknown suffix \"%c\"", suffix)
	}
	return val, err
}

func readRandomUndirectedGraph(orderstr string, size int) (BronKerbosch.UndirectedGraph, error) {
	order, err := parsePositiveInt(orderstr)
	if err != nil {
		return BronKerbosch.UndirectedGraph{}, err
	}
	fullyMeshedSize := order * (order - 1) / 2
	if size > fullyMeshedSize {
		panic(
			fmt.Sprintf("%d nodes accommodate at most %d edges", order, fullyMeshedSize))
	}

	name := "random_edges_order_" + orderstr + ".txt"
	path := filepath.Join(".", name)
	file, err := os.Open(path)
	if err != nil {
		err = fmt.Errorf("%s\nPerhaps generate it with `python -m ..\\python3\\random_graph %s <max_size?>`", err, orderstr)
		return BronKerbosch.UndirectedGraph{}, err
	}
	defer file.Close()
	adjacencies := make([]BronKerbosch.VertexSet, order)
	for v := 0; v < order; v++ {
		adjacencies[v] = make(BronKerbosch.VertexSet)
	}
	scanner := bufio.NewScanner(file)
	linenum := 0
	for scanner.Scan() {
		if linenum == size {
			break
		}
		linenum += 1
		line := scanner.Text()
		var v int
		var w int
		_, err := fmt.Sscanf(line, "%d %d", &v, &w)
		if err != nil {
			err = fmt.Errorf("%s in file %s at line %d", err, path, linenum)
			return BronKerbosch.UndirectedGraph{}, err
		}
		adjacencies[v].Add(BronKerbosch.Vertex(w))
		adjacencies[w].Add(BronKerbosch.Vertex(v))
	}
	if err := scanner.Err(); err != nil {
		return BronKerbosch.UndirectedGraph{}, err
	}

	if linenum < size {
		err = fmt.Errorf("Exhausted generated list of %d edges in %s", linenum, path)
		return BronKerbosch.UndirectedGraph{}, err
	}
	g := BronKerbosch.NewUndirectedGraph(adjacencies)
	if g.Order() != order {
		panic("botched order")
	}
	if g.Size() != size {
		panic("botched size")
	}
	return g, nil
}
