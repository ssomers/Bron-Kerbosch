package main

import (
	"bron_kerbosch"
	"fmt"
)

func bk(order int, sizes []int) {
	const SAMPLES = 3
	for _, size := range sizes {
		stats := bron_kerbosch.Timed(order, size, SAMPLES)
		for func_index, func_name := range bron_kerbosch.FUNC_NAMES {
			mean := stats[func_index].Mean()
			dev := stats[func_index].Deviation()
			fmt.Printf("order %7d size %7d %8s: %5.2fs %c%5.2fs\n", order, size, func_name, mean, 177, dev)
		}
	}
}

func main() {
	var sizes_100 []int
	var sizes_10k []int
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
	bk(100, sizes_100)
	bk(10000, sizes_10k)
}
