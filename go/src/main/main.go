package main

import (
	"BronKerbosch"
	"fmt"
	"os"
	"strconv"
)

func bk(orderstr string, order int, sizes []int, funcIndices []int, samples int) {
	name := "bron_kerbosch_go_order_" + orderstr
	path := name + ".csv"
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
		stats := BronKerbosch.Timed(order, size, funcIndices, samples)
		for _, funcIndex := range funcIndices {
			name := BronKerbosch.FuncNames[funcIndex]
			max := stats[funcIndex].Max()
			min := stats[funcIndex].Min()
			mean := stats[funcIndex].Mean()
			dev := stats[funcIndex].Deviation()
			fo.WriteString(fmt.Sprintf(",%f,%f,%f", min, mean, max))
			fmt.Printf("order %7d size %7d %-8s: %5.2fs %c%5.2fs\n", order, size, name, mean, 177, dev)
		}
		fo.WriteString("\n")
	}
}

func main() {
	allFuncIndices := []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	mtFuncIndices := []int{4, 6, 7, 8, 9}
	if len(os.Args) == 1 {
		var sizes_100 []int
		var sizes_10k []int
		var sizes_1M []int
		for s := int(2e3); s <= 3e3; s += 50 {
			sizes_100 = append(sizes_100, s)
		}
		for s := int(100e3); s <= 800e3; s += 100e3 {
			sizes_10k = append(sizes_10k, s)
		}
		for s := int(200e3); s <= 5e6; {
			sizes_1M = append(sizes_1M, s)
			if s < 2e6 {
				s += 200e3
			} else {
				s += 1e6
			}
		}
		bk("100", 1e2, sizes_100, allFuncIndices, 5)
		bk("10k", 1e4, sizes_10k, allFuncIndices, 3)
		bk("1M", 1e6, sizes_1M, mtFuncIndices, 3)
	} else if len(os.Args) > 2 {
		orderstr := os.Args[1]
		order, err := strconv.Atoi(orderstr)
		if err != nil {
			panic(err)
		}
		var sizes []int
		for _, s := range os.Args[2:] {
			size, err := strconv.Atoi(s)
			sizes = append(sizes, size)
			if err != nil {
				panic(err)
			}
		}
		bk(orderstr, order, sizes, allFuncIndices, 3)
	} else {
		print("give me one or more sizes too")
	}
}
