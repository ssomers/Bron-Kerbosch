package main

import (
	"bron_kerbosch"
	"fmt"
	"os"
)

func bk(orderstr string, order int, sizes []int) {
	const SAMPLES = 3
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
	for _, name := range bron_kerbosch.FUNC_NAMES {
		fo.WriteString(fmt.Sprintf(",%s min,%s mean,%s max", name, name, name))
	}
	fo.WriteString("\n")
	for _, size := range sizes {
		fo.WriteString(fmt.Sprintf("%d", size))
		stats := bron_kerbosch.Timed(order, size, SAMPLES)
		for func_index, func_name := range bron_kerbosch.FUNC_NAMES {
			max := stats[func_index].Max()
			min := stats[func_index].Min()
			mean := stats[func_index].Mean()
			dev := stats[func_index].Deviation()
			fo.WriteString(fmt.Sprintf(",%f,%f,%f", min, mean, max))
			fmt.Printf("order %7d size %7d %8s: %5.2fs %c%5.2fs\n", order, size, func_name, mean, 177, dev)
		}
		fo.WriteString("\n")
	}
}

func main() {
	var sizes_100 []int
	var sizes_10k []int
	var sizes_1M []int
	for s := 2000; s <= 3000; s += 50 {
		sizes_100 = append(sizes_100, s)
	}
	for s := 1000; s <= 200000; {
		sizes_10k = append(sizes_10k, s)
		if s < 10000 {
			s += 1000
		} else {
			s += 10000
		}
	}
	for s := 0; s <= 3e6; {
		sizes_1M = append(sizes_1M, s)
		if s < 1e6 {
			s += 0.25e6
		} else {
			s += 0.5e6
		}
	}
	bk("100", 1e2, sizes_100)
	bk("10k", 1e4, sizes_10k)
	bk("1M", 1e6, sizes_1M)
}
