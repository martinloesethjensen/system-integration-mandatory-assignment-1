# Running NemID Code Generation

## Locally with Rust Installed

```sh
cargo run
```

## Dockerfile

```sh
docker build -t nem_id_code_gen:v1 .
docker run -t nem_id_code_gen:v1       -------    TODO
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

Status code `200` (OK) and output with 5 random number like this.

```json
{"generatedCode": "{random_five_numbers}"}
```

But if the user does not exist and nemid or password is incorrect then the response would be like this.

Status code `403` (FORBIDDEN) and output is json with an error.

```json
{"error": "{error_description}"}
```
