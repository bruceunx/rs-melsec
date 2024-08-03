# rs-melsec

rs-melsec is a Rust implementation of the MELSEC Communication Protocol that allows you to interact with Mitsubishi PLCs. Inspired by the `pymcprotocol` and `pymelsec` libraries, `rs-melsec` aims to provide additional features and enhanced flexibility, while leveraging the safety and performance benefits of the Rust programming language.

## Installation

> Add `rs-melsec` to your `Cargo.toml`:

```toml
[dependencies]
rs-melsec = "0.1.0"
```

## Usage

```rust
use rs_melsec::client::Client;
use rs_melsec::db::DataType;
use rs_melsec::tag::QueryTag;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let host = args.get(1).expect("failed to get host");
    let default_port = 6000;
    let num_port = args
        .get(2)
        .and_then(|s| s.parse::<u16>().ok())
        .or(Some(default_port))
        .unwrap();

    let mut tags = Vec::new();
    tags.push(QueryTag {
        device: "M8304".to_string(),
        data_type: DataType::BIT,
    });
    let client = Client::new(host.to_string(), num_port, "iQ-R", true);
    let result = client.read(tags).expect("failed to read data");
    for tag in result {
        println!("{}", tag);
    }
}
```

> run `cargo run --bin host port`
