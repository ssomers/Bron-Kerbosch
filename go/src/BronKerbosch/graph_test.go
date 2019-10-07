package BronKerbosch

import (
	"testing"
)

func TestRandomGraph1(t *testing.T) {
	randomUndirectedGraph(2, 0)
	randomUndirectedGraph(3, 0)
	randomUndirectedGraph(3, 1)
	randomUndirectedGraph(3, 2)
	randomUndirectedGraph(4, 0)
	randomUndirectedGraph(4, 1)
	randomUndirectedGraph(4, 2)
	randomUndirectedGraph(4, 3)
	randomUndirectedGraph(4, 4)
	randomUndirectedGraph(4, 5)
}
