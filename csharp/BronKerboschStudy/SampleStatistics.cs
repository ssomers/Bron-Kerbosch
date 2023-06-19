namespace BronKerboschStudy
{
    public struct SampleStatistics
    {
        public double Max { get; private set; }
        public double Min { get; private set; }
        public int Samples { get; private set; }
        private double Sum;
        private double SumOfSquares;

        public readonly double Mean => Samples > 0
            ? Math.Max(Min, Math.Min(Max, Sum / Samples))
            : double.NaN;

        public readonly double Variance => Samples > 1
            ? Math.Max(0, SumOfSquares - Sum * Sum / Samples) / (Samples - 1)
            : double.NaN;

        public double Deviation => Math.Min(Max - Min, Math.Sqrt(Variance));

        public void Put(double v)
        {
            if (Samples == 0 || v < Min)
                Min = v;
            if (Samples == 0 || v > Max)
                Max = v;
            Samples += 1;
            Sum += v;
            SumOfSquares += v * v;
        }
    }
}
