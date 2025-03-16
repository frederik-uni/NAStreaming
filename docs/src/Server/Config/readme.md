# Config

```toml
[server]
host = "0.0.0.0"
port = 8080

[logging]
level = "info"
```
- `[server]` — Configures the server
  - `host` — Ip adress for the server
  - `port` — Port for the server
  - `https-port`— Port for https
- `[logging]`
  - `level` — What should be logged: ["trace", "debug", "info", "warn", "error"]
