package BronKerbosch

type Consumer struct {
	MinSize int
	Cliques chan []Vertex
}

func (c *Consumer) Add(clique []Vertex) {
	c.Cliques <- clique
}

func (c *Consumer) close() {
	close(c.Cliques)
}
