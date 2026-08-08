alias t := test
test:
  cargo test

alias l := lint
lint:
  cargo clippy

fmt:
  cargo fmt

fuzz TARGET *FLAGS:
  cargo +nightly fuzz run --fuzz-dir tests/fuzz {{FLAGS}} {{TARGET}}
