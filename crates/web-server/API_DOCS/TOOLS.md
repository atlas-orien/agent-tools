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

## EXTERNAL SEARCH PROVIDER

### ROUTE

`GET /api/external/search`

### PURPOSE

PROVIDE A CODEX-POWERED STRUCTURED SEARCH PROVIDER RESPONSE FOR AGENT SEARCH SKILLS.

### REQUEST

```bash
curl 'http://127.0.0.1:18080/api/external/search?q=latest%20Rust%20release&topic=news&days=30&max_results=6&bypass_cache=false'
```

### QUERY PARAMETERS

- `q`: REQUIRED SEARCH QUERY.
- `topic`: OPTIONAL SEARCH TOPIC. DEFAULTS TO `general`.
- `days`: OPTIONAL FRESHNESS WINDOW. DEFAULTS TO `30`, CLAMPED TO `1..365`.
- `max_results`: OPTIONAL SOURCE LIMIT. DEFAULTS TO `6`, CLAMPED TO `1..12`.
- `bypass_cache`: OPTIONAL BOOLEAN. DEFAULTS TO `false`.

### RESPONSE

```json
{
  "code": 0,
  "data": {
    "status": "live",
    "key": "search",
    "engine": "codex-web-search",
    "requested_at": "2026-05-25T12:00:00Z",
    "confidence": 0.75,
    "data": {
      "items": [
        {
          "title": "Source title",
          "snippet": "Short grounded source summary",
          "url": "https://example.com/source",
          "published_at": "2026-05-25"
        }
      ]
    }
  },
  "message": "Success"
}
```

### VALIDATION

- `q` MUST NOT BE EMPTY.
- CODEX OUTPUT MUST BE VALID JSON.
- CODEX OUTPUT MUST CONTAIN `data.items`.

### IMPLEMENTATION

- ROUTE: `src/routes/tools.rs`
- HANDLER: `src/handlers/tools/web_search.rs`
- LIB CALL: `agent_tools::external_search_provider_output`
