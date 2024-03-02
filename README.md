# Hello Axum!

This is an example app demonstrating how to deploy a Rust program using [Axum](https://github.com/tokio-rs/axum) on [Fly.io](https://fly.io/)

## Structure

### App

Axum-based "hello world" app using tokio.

#### Binding to all addresses

Noticed this lines?

```rust
    let addr : SocketAddr = "[::]:8080".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
```

This listens to all addresses on both IPv4 and IPv6, on port 8080. It's important to do this because your app would otherwise need to know about the `172.19.0.0/16` IP it should bind to specifically. Binding to IPv6 is not required, but is likely a good idea going forward.

### fly.toml

A simple `fly.toml` configuration with a few additional useful things

### Dockerfile

#### .dockerignore

You definitely want to ignore `/target` since this can get pretty hefty, adding to the build context's size, and slow down builds significantly.

## Deploy

```
fly launch
```

## Notes
There are some TODOs and NOTEs in the code. It's good to read them if you want to modify it for your use.

##### Based on https://github.com/fly-apps/hello-rust
