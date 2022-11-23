<h1 align="center">Interactive Brokers TWS API (Rust)</h1>

[![LGPL 3.0](https://img.shields.io/crates/l/ib_tws_core?style=for-the-badge)](https://choosealicense.com/licenses/lgpl-3.0/)

- Min. TWS API Version: 149

## Usage
### Tokio
```rust
let client = {
	let transport = ib_tws_tokio::Transport::connect(
		"127.0.0.1:4001".parse().unwrap(),
		Duration::from_secs(5),
	)
	.await?;
	ib_tws_core::AsyncClient::setup(transport, 0).await?
};
info!(version = client.server_version(), "connected to client");
```

## Crates
### [`ib_tws_core`](https://github.com/fourbytes/ib_tws_rs/tree/main/crates/ib_tws_core)
[![crates.io](https://img.shields.io/crates/v/ib_tws_core?style=for-the-badge)](https://crates.io/crates/ib_tws_core) [![docs.rs](https://img.shields.io/badge/docs.rs-ib_tws_core-rs?style=for-the-badge)](https://docs.rs/ib_tws_core)

Contains core components including messages and encoding/decoding utilities, as well as a high-level `AsyncClient`.

### [`ib_tws_tokio`](https://github.com/fourbytes/ib_tws_rs/tree/main/crates/ib_tws_tokio)
[![crates.io](https://img.shields.io/crates/v/ib_tws_tokio?style=for-the-badge)](https://crates.io/crates/ib_tws_core) [![docs.rs](https://img.shields.io/badge/docs.rs-ib_tws_tokio-rs?style=for-the-badge)](https://docs.rs/ib_tws_tokio)

A transport implementation using Tokio, intended to be used with the `ib_tws_core::AsyncClient`.

## Goals
- Keep as much common code in `ib_tws_core` as possible.
- Works on stable Rust.

## Credits
`ib_tws_core` is forked from [chrisdamba's ib_async](https://github.com/chrisdamba/ib_async).
