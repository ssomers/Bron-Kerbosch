using System.Globalization;

namespace BronKerboschStudy
{
    public sealed class NumbersGame
    {
        public static int ParseInt(string numstr)
        {
            int factor = 1;
            if (numstr.EndsWith('k'))
            {
                factor = 1_000;
                numstr = numstr[..^1];
            }
            else if (numstr.EndsWith('M'))
            {
                factor = 1_000_000;
                numstr = numstr[..^1];
            }
            return int.Parse(numstr, CultureInfo.InvariantCulture) * factor;
        }
    }
}
