package main

import (
	core "BronKerboschStudy/BronKerbosch"
	"BronKerboschStudy/Stats"
	"BronKerboschStudy/StudyIO"
	"fmt"
	"os"
	"path/filepath"
	"time"
)

func timed(orderstr string,
	size int,
	funcIndices []int,
	timedSamples int) [core.NumFuncs]Stats.SampleStatistics {
	begin := time.Now()
	graph, cliqueCount, err := StudyIO.ReadKnownRandomGraph(orderstr, size)
	secs := time.Since(begin).Seconds()
	if err != nil {
		panic(err)
	}
	fmt.Printf("order %4s size %7d %-8s: %5.3fs\n", orderstr, size, "creation", secs)

	var times [core.NumFuncs]Stats.SampleStatistics
	var first []core.Clique
	for sample := range timedSamples {
		for _, funcIndex := range funcIndices {
			bronKerboschFunc := core.Funcs[funcIndex]
			begin := time.Now()
			consumer := core.Consumer{}
			consumer.MinSize = 3
			consumer.Cliques = make(chan core.Clique, 64)
			go bronKerboschFunc(&graph, consumer)
			if sample == 0 {
				warning_interval := 3 * time.Second
				var ticker *time.Timer
				warnings := 0
				ticker = time.AfterFunc(warning_interval, func() {
					ticker.Reset(warning_interval)
					warnings += 1
					fmt.Printf("  %d seconds in, %s is still busy collecting\n",
						warnings*int(warning_interval.Seconds()),
						core.FuncNames[funcIndex])
				})
				var collectedCliques []core.Clique
				for clique := range consumer.Cliques {
					collectedCliques = append(collectedCliques, clique)
				}
				ticker.Stop()
				core.SortCliques(collectedCliques)
				if len(first) == 0 {
					if len(collectedCliques) != cliqueCount {
						fmt.Printf("  %s: expected %d cliques, obtained %d\n",
							core.FuncNames[funcIndex], cliqueCount, len(collectedCliques))
					}
					first = collectedCliques
				} else {
					core.CompareCliques(collectedCliques, first, func(e string) {
						fmt.Printf("  %s: %s\n", core.FuncNames[funcIndex], e)
					})
				}
			} else {
				var countedCliques int
				for range consumer.Cliques {
					countedCliques += 1
				}
				secs := time.Since(begin).Seconds()
				if countedCliques != cliqueCount {
					fmt.Printf("  %s: expected %d cliques, obtained %d\n",
						core.FuncNames[funcIndex], cliqueCount, countedCliques)
				}
				times[funcIndex].Put(secs)
			}
		}
	}
	return times
}

func bk(orderstr string, sizes []int, funcIndices []int, timedSamples int) {
	path := filepath.Join("..", "data", "random_time_go_order_"+orderstr+".csv")
	fo, err := os.Create(path)
	if err != nil {
		panic(err)
	}

	defer func() {
		if err := fo.Close(); err != nil {
			panic(err)
		}
	}()

	_, err = fo.WriteString("Size")
	if err != nil {
		panic(err)
	}
	for _, funcIndex := range funcIndices {
		name := core.FuncNames[funcIndex]
		_, err = fo.WriteString(fmt.Sprintf(",%s min,%s mean,%s max", name, name, name))
	}
	_, err = fo.WriteString("\n")
	if err != nil {
		panic(err)
	}
	for _, size := range sizes {
		_, err = fo.WriteString(fmt.Sprintf("%d", size))
		if err != nil {
			panic(err)
		}
		stats := timed(orderstr, size, funcIndices, timedSamples)
		for _, funcIndex := range funcIndices {
			name := core.FuncNames[funcIndex]
			max := stats[funcIndex].Max()
			min := stats[funcIndex].Min()
			mean := stats[funcIndex].Mean()
			dev := stats[funcIndex].Deviation()
			_, err = fo.WriteString(fmt.Sprintf(",%f,%f,%f", min, mean, max))
			if err != nil {
				panic(err)
			}
			fmt.Printf("order %4s size %7d %-8s: %5.3fs ± %.0f%%\n",
				orderstr, size, name, mean, 100*dev/mean)
		}
		_, err = fo.WriteString("\n")
		if err != nil {
			panic(err)
		}
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
		bk(orderstr, sizes, allFuncIndices, 0)
	} else {
		fmt.Println("give me one or more sizes too")
	}
}
