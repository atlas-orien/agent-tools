# TOOLS API

## ROUTE GROUP

`/tools`

TOOL ROUTES ARE REGISTERED THROUGH `src/routes/tools.rs`.

## WEB SEARCH

### ROUTE

`POST /tools/web-search`

### PURPOSE

RUN A CODEX-POWERED WEB SEARCH THROUGH `agent_tools::web_search`.

### REQUEST

```bash
curl -X POST http://127.0.0.1:18080/tools/web-search \
  -H 'content-type: application/json' \
  -d '{"query":"What is the latest stable Rust version?"}'
```

### REQUEST BODY

```json
{
  "query": "What is the latest stable Rust version?"
}
```

### RESPONSE

```json
{
  "code": 0,
  "data": {
    "answer": "..."
  },
  "message": "Success"
}
```

### VALIDATION

- `query` MUST NOT BE EMPTY.

### ERROR RESPONSE

```json
{
  "code": 400,
  "message": "query cannot be empty"
}
```

### IMPLEMENTATION

- ROUTE: `src/routes/tools.rs`
- HANDLER: `src/handlers/tools/web_search.rs`
- LIB CALL: `agent_tools::web_search`
