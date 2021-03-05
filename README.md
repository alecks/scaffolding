# Scaffolding

Scaffolding is a high-performance Discord bot client that can be accessed via gRPC. It aims to allow developers to take advantage of Rust's performance while using another language of their choice.

## Usage

Scaffolding can be configured with environment variables<sup>[1](#config-fn)</sup>. It is required that you proivide an address for the server to listen on via the `SERVER_ADDR` variable. This should be in the form of `host:port`, e.g. `0.0.0.0:50051`.

You may also specify a log level (for [tracing](https://github.com/tokio-rs/tracing)) with the `RUST_LOG` enviroment variable. It is recommended that this is set to `info`.

Once it is configured, simply run the binary:

```shell
$ scaffolding
```

---

<a name="config-fn">1</a>: This is subject to change; at current, its limited configuration options make a file-based config unnecessary.
