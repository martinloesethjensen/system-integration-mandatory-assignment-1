using Microsoft.AspNetCore.Mvc;
using User_Generator.Models;
using User_Generator.Utils;

namespace User_Generator.Controllers
{
    [Route("/generate-nemID")]
    [ApiController]
    public class UserGenerationController : ControllerBase
    {
        [HttpPost]
        public IActionResult Post([FromBody] User bodyPayload)
        {
            if (!Util.IsUserValid(bodyPayload)) 
            {
                return BadRequest("The body must consits 10 digits long 'cpr' and not empty 'email'...");
            }
            NemID Response = new NemID(bodyPayload.Cpr);
            return Created("", Response);
        }
    }
}
    