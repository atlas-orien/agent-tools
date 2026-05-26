# CONFIG

## DEFAULT CONFIG FILE

`config/services.toml`

THIS PATH IS RELATIVE TO THE CURRENT WORKING DIRECTORY USED TO START THE SERVER.

THIS FILE IS LOCAL-ONLY AND IGNORED BY GIT.

COMMIT THIS EXAMPLE FILE INSTEAD:

`config/services.example.toml`

## DEFAULT CONTENT

```toml
[http]
port = 18080
```

## START WITH DEFAULT CONFIG

CREATE THE LOCAL CONFIG FIRST:

```bash
cp config/services.example.toml config/services.toml
```

```bash
cargo run -p tools-server
```

## DEPLOYMENT NOTE

THE COMPILED SERVER BINARY CAN BE DEPLOYED WITH A SIBLING `config/services.toml` FILE:

```text
tools-server
config/
  services.toml
```
