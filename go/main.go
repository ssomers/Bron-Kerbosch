package main

import (
	"BronKerboschStudy/BronKerbosch"
	"BronKerboschStudy/Stats"
	"BronKerboschStudy/StudyIO"
	"fmt"
	"os"
	"path/filepath"
	"time"
)

func timed(orderstr string, size int, funcIndices []int, samples int) [BronKerbosch.NumFuncs]Stats.SampleStatistics {
	begin := time.Now()
	graph, cliqueCount, err := StudyIO.ReadRandomUndirectedGraph(orderstr, size)
	secs := time.Since(begin).Seconds()
	if err != nil {
		panic(err)
	}
	fmt.Printf("order %4s size %7d %-8s: %5.3fs\n", orderstr, size, "creation", secs)

	var times [BronKerbosch.NumFuncs]Stats.SampleStatistics
	var first [][]BronKerbosch.Vertex
	sample := 0
	if samples == 1 {
		sample = 1
	}
	for ; sample <= samples; sample++ {
		for _, funcIndex := range funcIndices {
			bronKerboschFunc := BronKerbosch.Funcs[funcIndex]
			if sample == 0 {
				var reporter BronKerbosch.SimpleReporter
				begin := time.Now()
				bronKerboschFunc(&graph, &reporter)
				secs := time.Since(begin).Seconds()
				if secs >= 3.0 {
					fmt.Printf("  %-8s: %5.2fs\n", BronKerbosch.FuncNames[funcIndex], secs)
				}
				current := reporter.Cliques
				BronKerbosch.SortCliques(current)
				if len(first) == 0 {
					if len(current) != cliqueCount {
						fmt.Printf("  %s: expected %d cliques, obtained %d\n",
							BronKerbosch.FuncNames[funcIndex], cliqueCount, len(current))
					}
					first = current
				} else {
					BronKerbosch.CompareCliques(current, first, func(e string) {
						fmt.Printf("  %s: %s\n", BronKerbosch.FuncNames[funcIndex], e)
					})
				}
			} else {
				var reporter BronKerbosch.CountingReporter
				begin := time.Now()
				bronKerboschFunc(&graph, &reporter)
				secs := time.Since(begin).Seconds()
				if reporter.Cliques != cliqueCount {
					fmt.Printf("  %s: expected %d cliques, obtained %d\n",
						BronKerbosch.FuncNames[funcIndex], cliqueCount, reporter.Cliques)
				}
				times[funcIndex].Put(secs)
			}
		}
	}
	return times
}

func bk(orderstr string, sizes []int, funcIndices []int, samples int) {
	name := "bron_kerbosch_go_order_" + orderstr
	path := filepath.Join("..", name+".csv")
	fo, err := os.Create(path)
	if err != nil {
		panic(err)
	}

	defer func() {
		if err := fo.Close(); err != nil {
			panic(err)
		}
	}()

	fo.WriteString("Size")
	for _, funcIndex := range funcIndices {
		name := BronKerbosch.FuncNames[funcIndex]
		fo.WriteString(fmt.Sprintf(",%s min,%s mean,%s max", name, name, name))
	}
	fo.WriteString("\n")
	for _, size := range sizes {
		fo.WriteString(fmt.Sprintf("%d", size))
		stats := timed(orderstr, size, funcIndices, samples)
		for _, funcIndex := range funcIndices {
			name := BronKerbosch.FuncNames[funcIndex]
			max := stats[funcIndex].Max()
			min := stats[funcIndex].Min()
			mean := stats[funcIndex].Mean()
			dev := stats[funcIndex].Deviation()
			fo.WriteString(fmt.Sprintf(",%f,%f,%f", min, mean, max))
			fmt.Printf("order %4s size %7d %-8s: %5.3fs ± %.0f%%\n", orderstr, size, name, mean, 100*dev/mean)
		}
		fo.WriteString("\n")
	}
}

func main() {
	allFuncIndices := []int{0, 1, 2, 3, 4, 5, 6, 7, 8}
	mostFuncIndices := []int{1, 2, 3, 4, 5, 6, 7, 8}
	mtFuncIndices := []int{3, 4, 5, 6, 7, 8}
	if len(os.Args) == 1 {
		var sizes_100 []int
		var sizes_10k []int
		var sizes_1M []int
		for s := int(2e3); s <= 3e3; s += 50 {
			sizes_100 = append(sizes_100, s)
		}
		for s := int(10e3); s <= 200e3; {
			sizes_10k = append(sizes_10k, s)
			if s < 100e3 {
				s += 10e3
			} else {
				s += 25e3
			}
		}
		for s := int(500e3); s <= 5e6; {
			sizes_1M = append(sizes_1M, s)
			if s < 2e6 {
				s += 250e3
			} else {
				s += 1e6
			}
		}
		bk("100", sizes_100, allFuncIndices, 5)
		bk("10k", sizes_10k, mostFuncIndices, 3)
		bk("1M", sizes_1M, mtFuncIndices, 3)
	} else if len(os.Args) > 2 {
		orderstr := os.Args[1]
		var sizes []int
		for _, sizestr := range os.Args[2:] {
			size, err := StudyIO.ParsePositiveInt(sizestr)
			if err != nil {
				panic(err)
			}
			sizes = append(sizes, size)
		}
		bk(orderstr, sizes, allFuncIndices, 3)
	} else {
		print("give me one or more sizes too")
	}
}
