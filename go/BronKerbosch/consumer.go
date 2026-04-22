package BronKerbosch

type Consumer struct {
	MinSize int
	Accept  func(Clique)
}

func (c *Consumer) Add(clique Clique) {
	c.Accept(clique)
}
