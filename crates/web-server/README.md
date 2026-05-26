# tools-server

Small HTTP server for `agent-tools`.

## Run

```bash
cargo run -p tools-server -- --bind 0.0.0.0:18080
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

External search provider:

```bash
curl 'http://127.0.0.1:18080/api/external/search?q=latest%20Rust%20release&topic=news&days=30&max_results=6&bypass_cache=false'
```
