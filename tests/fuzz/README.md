# Fuzzing js.rs

## Setup

Get [`cargo-fuzz`](https://github.com/rust-fuzz/cargo-fuzz) (nightly needed).

```sh
rustup install nightly
cargo +nightly install cargo-fuzz
```

## Run

Run `just fuzz <TARGET>` where `<TARGET>` is a file in `fuzz_targets/`, for example `just fuzz lexer`.
