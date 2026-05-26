# tools-server

Small HTTP server for `agent-tools`.

## Run

```bash
cargo run -p tools-server
```

Default config file: `config/services.toml`.

Create it from the example:

```bash
cp config/services.example.toml config/services.toml
```

## Routes

Health:

```bash
curl http://127.0.0.1:18080/health
```

Web search:

```bash
curl -X POST http://127.0.0.1:18080/tools/web-search \
  -H 'content-type: application/json' \
  -d '{"query":"What is the latest stable Rust version?"}'
```
