package main

import (
	"BronKerbosch/lib"
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

func readRandomUndirectedGraph(orderstr string, size int) (BronKerbosch.UndirectedGraph, int, error) {
	order, err := parsePositiveInt(orderstr)
	if err != nil {
		return BronKerbosch.UndirectedGraph{}, 0, err
	}
	fullyMeshedSize := order * (order - 1) / 2
	if size > fullyMeshedSize {
		panic(
			fmt.Sprintf("%d nodes accommodate at most %d edges", order, fullyMeshedSize))
	}

	edgesName := "random_edges_order_" + orderstr + ".txt"
	statsName := "random_stats.txt"
	edgesPath := filepath.Join("..", "data", edgesName)
	statsPath := filepath.Join("..", "data", statsName)
	adjacencies, err := readEdges(edgesPath, orderstr, order, size)
	if err != nil {
		err = fmt.Errorf("%s\nPerhaps generate it with `python -m ..\\python3\\random_graph %s <max_size?>`", err, orderstr)
		return BronKerbosch.UndirectedGraph{}, 0, err
	}
	cliqueCount, err := readStats(statsPath, orderstr, size)
	if err != nil {
		return BronKerbosch.UndirectedGraph{}, 0, err
	}

	g := BronKerbosch.NewUndirectedGraph(adjacencies)
	if g.Order() != order {
		panic("botched order")
	}
	if g.Size() != size {
		panic("botched size")
	}
	return g, cliqueCount, nil
}

func readEdges(path string, orderstr string, order int, size int) ([]BronKerbosch.VertexSet, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
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
			return nil, err
		}
		adjacencies[v].Add(BronKerbosch.Vertex(w))
		adjacencies[w].Add(BronKerbosch.Vertex(v))
	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}

	if linenum < size {
		err = fmt.Errorf("Exhausted generated list of %d edges in %s", linenum, path)
		return nil, err
	}
	return adjacencies, nil
}

func readStats(path string, orderstr string, size int) (int, error) {
	file, err := os.Open(path)
	if err != nil {
		return 0, err
	}
	defer file.Close()
	format := fmt.Sprintf("%s\t%d\t%%d", orderstr, size)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		var c int
		_, err := fmt.Sscanf(line, format, &c)
		if err == nil {
			return c, nil
		}
	}
	if err := scanner.Err(); err != nil {
		return 0, err
	}
	return 0, fmt.Errorf("File %s lacks order %s size %d", path, orderstr, size)
}
