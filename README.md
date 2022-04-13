# gorps
GURPS character management with go.

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
### `/api/v1/login`
