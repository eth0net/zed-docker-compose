# zed-docker-compose

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
