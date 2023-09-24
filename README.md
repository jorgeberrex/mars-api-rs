# mars-api-rs

Web API compatible with [Mars](https://github.com/Warzone/mars)

## Building

This project is written in Rust, so building is done via `cargo`. To build in release mode:

```
cargo build --release
```

The resulting program can be found in `target/release/mars_api_rs`. See [the reference implementation](https://github.com/Warzone/mars-api) for instructions on configuration and running.

## Notes

Currently, the websocket listens on port 7000 and the HTTP API listens on port 8000. This can be changed using the environment variables `MARS_WS_PORT` and `MARS_HTTP_PORT` respectively.
