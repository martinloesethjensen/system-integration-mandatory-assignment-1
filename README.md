
# System Integration Assignment 1

This is the first mandatory assignment for System Integration.
Have a look at the assignment description [here](/Mandatory_1_SD20w2.pdf).

## Prerequisites

- Python 3.7 or above
  - sqlite3
  - requests
  - json
  - pandas
  - xml-python
  - msgpack

To install these packages run this command: sudo pip install sqlite3 requests json pandas xml-python msgpack

- NodeJS 14.5 or above and NPM
  - express
  - axios
  - express-xml-bodyparser
  - sqlite3
  - readline-sync
  - request

To install these packages run this command:

```sh
npm install express axios express-xml-bodyparser sqlite3 readline-sync request
```

Have dart installed.
You can get dart via this [link](https://dart.dev/get-dart).

---

## Running the Client CLI System

In the root project:

```sh
cd Client_System; python3 client.py
```

## Running the NEMID ESB Server

In the root project:

```sh
cd NemID_ESB; node esb_mock.js
```

## Running the Main System

> This should only be run after ESB server is running.

In the root project:

```sh
cd Main_System; dart main.dart
```
