alias t := test
test:
  cargo test

alias l := lint
lint:
  cargo clippy

fmt:
  cargo fmt
