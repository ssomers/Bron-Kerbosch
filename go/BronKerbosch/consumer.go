package BronKerbosch

type Consumer struct {
	MinSize int
	Cliques chan Clique
}

func (c *Consumer) Add(clique Clique) {
	c.Cliques <- clique
}

func (c *Consumer) close() {
	close(c.Cliques)
}
