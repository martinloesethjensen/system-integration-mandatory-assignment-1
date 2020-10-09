using User_Generator.Models;

namespace User_Generator.Utils
{
    public class Util
    {
        public static bool IsUserValid(User userObj) 
        {
            if (string.IsNullOrWhiteSpace(userObj.Cpr) || string.IsNullOrWhiteSpace(userObj.Email) || userObj.Cpr.Length != 10) 
            {
                return false;
            }
            return true;
        }
    }
}
