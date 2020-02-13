using System;


namespace BronKerboschStudy
{
    public struct SampleStatistics
    {
        public double Max { get; private set; }
        public double Min { get; private set; }
        public int Samples { get; private set; }
        private double Sum;
        private double SumOfSquares;
        public double Mean
        {
            get
            {
                if (Samples > 0)
                {
                    var r = Sum / Samples;
                    return Math.Max(Min, Math.Min(Max, r));
                }
                else
                    return Double.NaN;
            }
        }
        public double Variance
        {
            get
            {
                if (Samples > 1)
                    return Math.Max(SumOfSquares - Sum * Sum / Samples, 0) / (Samples - 1);
                else
                    return Double.NaN;
            }
        }
        public double Deviation
        {
            get
            {
                return Math.Min(Max - Min, Math.Sqrt(Variance));
            }
        }

        public void Put(double v)
        {
            if (Samples == 0)
            {
                Min = v;
                Max = v;
            }
            else if (Min > v)
                Min = v;
            else if (Max < v)
                Max = v;
            Samples += 1;
            Sum += v;
            SumOfSquares += v * v;
        }

    }
}
