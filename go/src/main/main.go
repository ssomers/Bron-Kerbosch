package main

import (
	"BronKerbosch"
	"fmt"
	"os"
	"strconv"
)

func bk(orderstr string, order int, sizes []int, samples int) {
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
	for _, name := range BronKerbosch.FuncNames {
		fo.WriteString(fmt.Sprintf(",%s min,%s mean,%s max", name, name, name))
	}
	fo.WriteString("\n")
	for _, size := range sizes {
		fo.WriteString(fmt.Sprintf("%d", size))
		stats := BronKerbosch.Timed(order, size, samples)
		for func_index, func_name := range BronKerbosch.FuncNames {
			max := stats[func_index].Max()
			min := stats[func_index].Min()
			mean := stats[func_index].Mean()
			dev := stats[func_index].Deviation()
			fo.WriteString(fmt.Sprintf(",%f,%f,%f", min, mean, max))
			fmt.Printf("order %7d size %7d %-8s: %5.2fs %c%5.2fs\n", order, size, func_name, mean, 177, dev)
		}
		fo.WriteString("\n")
	}
}

func main() {
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
			if s < 1e6 {
				s += 200e3
			} else {
				s += 1e6
			}
		}
		bk("100", 1e2, sizes_100, 5)
		bk("10k", 1e4, sizes_10k, 3)
		bk("1M", 1e6, sizes_1M, 3)
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
		bk(orderstr, order, sizes, 3)
	} else {
		print("give me one or more sizes too")
	}
}
