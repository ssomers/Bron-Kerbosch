using Graph;
using System.Collections.Generic;
using System.Diagnostics.Contracts;

namespace BronKerbosch
{
    public interface Reporter
    {
        void record(List<Vertex> clique);
    }

    public class SimpleReporter : Reporter
    {
        private readonly List<List<Vertex>> cliques = new List<List<Vertex>>();

        public List<List<Vertex>> Cliques { get => cliques; }

        public virtual void record(List<Vertex> clique)
        {
            Contract.Requires(clique.Count > 1);
            Cliques.Add(clique);
        }
    }
}
