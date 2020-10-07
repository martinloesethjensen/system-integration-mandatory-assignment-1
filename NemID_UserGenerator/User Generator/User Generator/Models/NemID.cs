using System;
using System.Text;

namespace User_Generator.Models
{
    public class NemID
    {
        public string NemId { get;}

        public NemID(string cpr)
        {
            NemId = GenerateNemId(cpr);     
        }

        private string GenerateNemId(string cpr) 
        {
            Random rnd = new Random();
            StringBuilder builder = new StringBuilder(5);

            for (int i = 0; i < 5; i++ ) 
            {
                builder.Append(rnd.Next(0,9));
            }

            return builder.ToString() + "-" + cpr.Substring(cpr.Length-4);
        }
    }
}
