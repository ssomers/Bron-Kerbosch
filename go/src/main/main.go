package main

import (
	"bron_kerbosch"
	"fmt"
	"os"
)

func bk(orderstr string, order int, sizes []int) {
	const SAMPLES = 3
	name := "bron_kerbosch_go_order_" + orderstr
	path := "../" + name + ".csv"
	fo, err := os.Create(path)
	if err != nil {
		panic(err)
	}

	defer func() {
		if err := fo.Close(); err != nil {
			panic(err)
		}
	}()

	if _, err := fo.Write([]byte("Size,Ver1 min,Ver1 max,Ver1 mean\n")); err != nil {
		panic(err)
	}
	for _, size := range sizes {
		stats := bron_kerbosch.Timed(order, size, SAMPLES)
		if _, err := fo.Write([]byte(fmt.Sprintf("%d,%f,%f,%f\n", size, stats[0].Min(), stats[0].Max(), stats[0].Mean()))); err != nil {
			panic(err)
		}
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
	bk("100", 100, sizes_100)
	bk("100k", 10000, sizes_10k)
}
