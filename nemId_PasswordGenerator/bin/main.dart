
import 'package:aqueduct/aqueduct.dart';

import '../lib/channel.dart';
import '../lib/nemid_passwordgenerator.dart';

Future main() async {
  final app = Application<NemIdPassGenChannel>()
      ..options.configurationFilePath = "config.yaml"
      ..options.port = 8089;

  final count = Platform.numberOfProcessors ~/ 2;
  await app.start(numberOfInstances: count > 0 ? count : 1);

  print("Application started on port: ${app.options.port}.");
  print("Use Ctrl-C (SIGINT) to stop running the application.");
}