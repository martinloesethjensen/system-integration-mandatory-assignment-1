using Microsoft.AspNetCore.Mvc;
using User_Generator.Models;

namespace User_Generator.Controllers
{
    [Route("/generate-nemID")]
    [ApiController]
    public class UserGenerationController : ControllerBase
    {
        [HttpPost]
        public IActionResult Post([FromBody] User bodyPayload)
        {
            NemID Response = new NemID(bodyPayload.Cpr);
            return Created("", Response);
        }
    }
}
    