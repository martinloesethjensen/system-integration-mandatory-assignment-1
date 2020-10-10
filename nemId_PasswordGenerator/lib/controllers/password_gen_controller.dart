import 'package:aqueduct/aqueduct.dart';
import 'package:nemid_passwordgenerator/nemid_passwordgenerator.dart';

class PasswordGenController extends ResourceController {
  @Operation.post()
  Future<Response> genPass() async {
    final Map<String, dynamic> bodyMap = request.body.as();
    final String nemid = bodyMap["nemId"] as String;
    final String cpr = bodyMap["cpr"] as String;
    return Response.ok({
      "nemIdPassword":
          "${nemid.substring(0, 2)}${cpr.substring(cpr.length - 2, cpr.length)}"
    })
      ..contentType = ContentType.json;
  }
}
