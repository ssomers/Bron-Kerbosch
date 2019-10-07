package bron_kerbosch

import (
	"testing"
)

func TestRandomGraph1(t *testing.T) {
	random_undirected_graph(2, 0)
	random_undirected_graph(3, 0)
	random_undirected_graph(3, 1)
	random_undirected_graph(3, 2)
	random_undirected_graph(4, 0)
	random_undirected_graph(4, 1)
	random_undirected_graph(4, 2)
	random_undirected_graph(4, 3)
	random_undirected_graph(4, 4)
	random_undirected_graph(4, 5)
}
