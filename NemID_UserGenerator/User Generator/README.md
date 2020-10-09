# Running NemID User Generation

## Locally with .NET Installed

Prerequisites
.NET Core 3.1 SDK or later

```sh
dotnet build
dotnet run

To specify port the service listening on
dotnet run --urls="http://localhost:8088"
```

## Dockerfile

```sh
docker build -t nem_id_user_gen .
docker run -p 8088:80 -t nem_id_user_gen
```

## Testing the Service

When the service is up and running then you can test it in the terminal with a simple `curl` command.

### Generate NemID

```sh
curl -X POST -H "Content-Type: application/json" \
    -d '{"cpr":"1234561234", "email": "m@m.com"}' \
    http://127.0.0.1:8088/generate-nemID
```

The out put should be like this.

```json
{"nemId": "{random_five_numbers}-{four_last_digits_of_cpr}"}
```

 ## API documentation
 ```sh
 Find Swagger documentation on
 http://localhost:8088/swagger/index.html
 ```
