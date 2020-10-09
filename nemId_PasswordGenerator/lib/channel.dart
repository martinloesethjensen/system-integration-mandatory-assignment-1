import 'controllers/password_gen_controller.dart';
import 'nemid_passwordgenerator.dart';

class NemIdPassGenChannel extends ApplicationChannel {
  @override
  Future prepare() async {
    logger.onRecord.listen(
        (rec) => print("$rec ${rec.error ?? ""} ${rec.stackTrace ?? ""}"));
  }

  @override
  Controller get entryPoint {
    final router = Router();

    router
        .route("/generate-password-nemID")
        .link(() => PasswordGenController());
    return router;
  }
}
