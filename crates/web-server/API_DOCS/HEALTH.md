# HEALTH API

## ROUTE

`GET /health`

## PURPOSE

CHECK WHETHER THE SERVER IS RUNNING.

## REQUEST

```bash
curl http://127.0.0.1:18080/health
```

## RESPONSE

```json
{
  "code": 0,
  "data": {
    "status": "ok"
  },
  "message": "Success"
}
```

## IMPLEMENTATION

- ROUTE: `src/routes/mod.rs`
- HANDLER: `src/handlers/health.rs`
