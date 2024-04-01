# simply

Just quickly start a very simple API service, which can save any request data to MongoDB, collection name is the path.


# Quick Start

## Just Download Binary

...

## From Source

Install Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone it

```shell
git clone git@github.com:jeremyyin2012/simply.git
```

Go into project dir

```shell
cd simply
```

Set .env
```shell
cp .env.example .env
```

Development Run

```shell
cargo run
```

Build Release

```shell
cargo build -r
```

Release Run

```shell
./target/release/simply
```
