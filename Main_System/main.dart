import 'dart:convert';
import 'dart:io';
import 'dart:math';

import 'package:csv/csv.dart';
import 'package:http/http.dart' as http;

main(List<String> args) async {
  // Reads the input we have from the `people.csv`
  final input = File('people.csv').openRead();
  final fields = await input
      .transform(utf8.decoder)
      .transform(CsvToListConverter())
      .toList();

  // Remove the column names
  fields.removeAt(0);

  // Map to Person objects
  final people = fields
      .map(
        (e) => Person(
          firstName: e[0],
          lastName: e[1],
          email: e[2],
          birthDate: e[3],
          country: e[4],
          address: e[5],
          phone: e[6],
        ),
      )
      .toList();

  final url = 'http://localhost:8080/nemID';

  people.forEach((person) async {
    final result = await http.post(url,
        body: person.toXml, headers: {'content-type': 'application/xml'});

    print(result.statusCode);
    print(result.body);

    if (result.statusCode == 200) {
      final jsonRespone = jsonDecode(result.body);
      person.nemId = jsonRespone['nemID'];
      print(person.nemId);
    }
  });
}

class Person {
  final firstName, lastName, email, birthDate, country, address, phone;
  String cpr;
  String nemId;

  Person({
    this.firstName,
    this.lastName,
    this.email,
    this.birthDate,
    this.country,
    this.address,
    this.phone,
  }) {
    // Generate CPR upon initialization
    this.cpr = generateCpr;
  }
}

extension on Person {
  String get generateCpr {
    final random = Random();
    final max = 9;
    return this.birthDate.split('-').map((e) {
          if (e.length > 2) {
            e = e.substring(2);
          }
          return e;
        }).join() +
        '${random.nextInt(max)}${random.nextInt(max)}${random.nextInt(max)}${random.nextInt(max)}';
  }

  String get toXml => '''<?xml version="1.0"?>
<Person>
  <FirstName>${this.firstName}</FirstName>
  <LastName>${this.lastName}</LastName>
  <CprNumber>${this.cpr}</CprNumber>
  <Email>${this.email}</Email>
</Person>''';
}
