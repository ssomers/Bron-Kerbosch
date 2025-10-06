using System;
using System.Globalization;

namespace BronKerboschStudy
{
    public sealed class NumbersGame
    {
        public static int ParseInt(string orderstr)
        {
            int factor = 1;
            if (orderstr.EndsWith("M", StringComparison.Ordinal))
            {
                factor = 1_000_000;
                orderstr = orderstr.Remove(orderstr.Length - 1);
            }
            else if (orderstr.EndsWith("k", StringComparison.Ordinal))
            {
                factor = 1_000;
                orderstr = orderstr.Remove(orderstr.Length - 1);
            }
            return int.Parse(orderstr, CultureInfo.InvariantCulture) * factor;
        }
    }
}
