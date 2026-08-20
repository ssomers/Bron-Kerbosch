using System;
using System.Globalization;

namespace BronKerboschStudy
{
    internal static class NumbersGame
    {
        public static int ParseInt(string numstr)
        {
            int factor = 1;
            if (numstr.EndsWith('k', StringComparison.Ordinal))
            {
                factor = 1_000;
                numstr = numstr[..^1];
            }
            else if (numstr.EndsWith('M', StringComparison.Ordinal))
            {
                factor = 1_000_000;
                numstr = numstr[..^1];
            }
            return int.Parse(numstr, CultureInfo.InvariantCulture) * factor;
        }
    }
}
