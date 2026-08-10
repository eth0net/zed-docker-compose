# zed-docker-compose

> [!IMPORTANT]
> **This extension is retired.** Docker Compose support now lives in the
> [Dockerfile extension](https://github.com/zed-extensions/dockerfile), which
> already ships the same language server — see
> [zed-extensions/dockerfile#48](https://github.com/zed-extensions/dockerfile/pull/48).
>
> Install **Dockerfile** from Zed's extensions store instead. The Compose
> language, highlight queries and per-service runnables all moved across.
>
> The version published here never advanced past 0.1.0. See
> [#10](https://github.com/eth0net/zed-docker-compose/issues/10) for the full
> history.

A [Zed](https://zed.dev) extension providing language server support for
Docker Compose files.

Applies to `compose.yaml`, `compose.yml`, `docker-compose.yaml` and
`docker-compose.yml`, which are given their own `Docker Compose` language so
that plain YAML files are left alone.

## Installation

Install from the integrated Zed extensions store.

The language server is downloaded automatically on first use. To use your own
build instead, put `docker-language-server` on your `$PATH` and it will be
preferred over the downloaded copy.

## Configuration

Telemetry is disabled by default. The server's other options can be set through
Zed's settings:

```json
{
  "lsp": {
    "docker-language-server": {
      "initialization_options": {
        "telemetry": "off"
      }
    }
  }
}
```

## Credits

Implemented using
[docker/docker-language-server](https://github.com/docker/docker-language-server),
the official language server from Docker.
