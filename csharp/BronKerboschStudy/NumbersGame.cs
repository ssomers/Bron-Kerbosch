using System.Globalization;

namespace BronKerboschStudy
{
    public sealed class NumbersGame
    {
        public static int ParseInt(string orderstr)
        {
            int factor = 1;
            if (orderstr.EndsWith('k'))
            {
                factor = 1_000;
                orderstr = orderstr[..^1];
            }
            else if (orderstr.EndsWith('M'))
            {
                factor = 1_000_000;
                orderstr = orderstr[..^1];
            }
            return int.Parse(orderstr, CultureInfo.InvariantCulture) * factor;
        }
    }
}
