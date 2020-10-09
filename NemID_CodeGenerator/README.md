# Running NemID Code Generator

## Locally with Rust Installed

> If rust is not installed, then you can install it [here](https://www.rust-lang.org/tools/install)
> You can get `rustup` that has `rustc` and `cargo` with a simple curl command: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

```sh
cargo run
```

## Dockerfile

> Currently the rust server tries to access the database file from [nem id esb db file](../NemID_ESB/nem_id_database.sqlite) and this can not be accessed by the docker container. Therefore, the rust server should be run locally.

```sh
docker build -t nem_id_code_gen .
docker run -it --rm --name running_nem_id_code_gen -p 8090 nem_id_code_gen bash -c "cargo run"
```

## Testing the Service

When the service is up and running then you can test it in the terminal with a simple `curl` command.

### Authenticate NemID

```sh
curl -X POST -H "Content-Type: application/json" \
    -d '{"nemIdCode": "9931", "nemId": "999681231"}' \
    http://127.0.0.1:8090/nemid-auth
```

If user exists in the db and nemid and password is correct, then the response would be this.

Status code `200` (OK) and output with 6 random number like this.

```json
{"generatedCode": "{random_six_numbers}"}
```

But if the user does not exist and nemid or password is incorrect then the response would be like this.

Status code `403` (FORBIDDEN) and output is json with an error.

```json
{"error": "{error_description}"}
```
