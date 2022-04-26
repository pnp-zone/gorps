# goRpS
GURPS character management as go web server with rust frontend.

## Building
```bash
go build -o bin/ ./...
```

## Configuration
There's an example configuration file located at `example.config.toml`.
Gorps-server is searching for a file with the name `config.toml` in its CWD. 

As an alternative, specify the path to the configuration file by starting the server with the parameter
`--config-path /path/to/config.toml`

## Run
```bash
bin/./gorps-server
```

## API

### `/api/v1/register`
- Method: `POST`

Example body:
```json
{
  "username": "",
  "password": ""
}
```

### `/api/v1/login`
- Method: `POST`

Example body:
```json
{
  "username": "",
  "password": ""
}
```

### `/api/v1/logout`
- Method `POST`

Body is not required.
