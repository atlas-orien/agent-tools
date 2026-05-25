# TOOLS SERVER API DOCS

THIS DIRECTORY IS FOR AI-FACING API NOTES.

THE SERVER CURRENTLY EXPOSES A SMALL HTTP API FOR `agent-tools`.

## RESPONSE FORMAT

SUCCESS RESPONSES USE THE `toolcraft-axum-kit` COMMON RESPONSE SHAPE:

```json
{
  "code": 0,
  "data": {},
  "message": "Success"
}
```

ERROR RESPONSES USE:

```json
{
  "code": 400,
  "message": "query cannot be empty"
}
```

## ROUTE LAYOUT

`/health` IS A ROOT ROUTE.

TOOL ROUTES ARE GROUPED UNDER `/tools`.

CURRENT ROUTES:

- `GET /health`
- `POST /tools/web-search`

## HEALTH

REQUEST:

```bash
curl http://127.0.0.1:18080/health
```

RESPONSE:

```json
{
  "code": 0,
  "data": {
    "status": "ok"
  },
  "message": "Success"
}
```

## WEB SEARCH

REQUEST:

```bash
curl -X POST http://127.0.0.1:18080/tools/web-search \
  -H 'content-type: application/json' \
  -d '{"query":"What is the latest stable Rust version?"}'
```

REQUEST BODY:

```json
{
  "query": "What is the latest stable Rust version?"
}
```

RESPONSE:

```json
{
  "code": 0,
  "data": {
    "answer": "..."
  },
  "message": "Success"
}
```

VALIDATION:

- `query` MUST NOT BE EMPTY.

IMPLEMENTATION NOTES:

- THE HANDLER CALLS `agent_tools::web_search`.
- THE ROUTE IS REGISTERED IN `src/routes/tools.rs`.
- THE HANDLER LIVES IN `src/handlers/tools/web_search.rs`.
