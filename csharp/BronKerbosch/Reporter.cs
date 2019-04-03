using BronKerbosch;
using System.Collections.Generic;
using System.Diagnostics.Contracts;

namespace BronKerbosch
{
    public interface Reporter
    {
        void Record(List<Vertex> clique);
    }

    public sealed class SimpleReporter : Reporter
    {
        public List<List<Vertex>> Cliques { get; } = new List<List<Vertex>>();

        public void Record(List<Vertex> clique)
        {
            Contract.Requires(clique.Count > 1);
            Cliques.Add(clique);
        }
    }
}
