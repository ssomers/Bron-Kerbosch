import base.{Clique, Vertex}

import scala.collection.mutable.Set

object bron_kerbosch1 {
    def explore(graph: UndirectedGraph, reporter: Reporter): Unit = {
        val candidates: collection.mutable.Set[Vertex] = graph.connected_nodes()
        visit(graph, reporter, candidates, collection.mutable.Set.empty[Vertex], // ::with_capacity(num_candidates),
            List[Vertex]())
    }

    def visit(graph: UndirectedGraph, reporter: Reporter, candidates: collection.mutable.Set[Vertex], excluded: collection.mutable.Set[Vertex], clique: Clique): Unit = {
        assert(candidates.forall(v => graph.degree(v) > 0))
        assert(excluded.forall(v => graph.degree(v) > 0))
        if (candidates.isEmpty && excluded.isEmpty) {
            reporter.record(clique)
            return
        }

        while (!candidates.isEmpty) {
            val v = pop_arbitrary(candidates)
            val neighbours = graph.neighbours(v)
            visit(graph, reporter, candidates & neighbours, excluded & neighbours, clique ++ List(v))
            excluded += v
        }
    }

    def pop_arbitrary(s: Set[Vertex]): Vertex = {
        val v = s.head
        s.remove(v)
        v
    }
}
