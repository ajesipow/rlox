DEFAULT_RUST_NIGHTLY_TOOLCHAIN_VERSION := '2024-05-21'

nightly_toolchain_version := DEFAULT_RUST_NIGHTLY_TOOLCHAIN_VERSION

all: format lint

[private]
alias a := all
[private]
alias c := check
[private]
alias l := lint
[private]
alias f := format

lint: \
  lint-clippy \
  lint-fmt

check:
    cargo c

format: update-nightly
  cargo +nightly-{{nightly_toolchain_version}}  fmt --all

lint-clippy:
    cargo clippy --all-targets -- -D warnings

lint-fmt: update-nightly
    cargo +nightly-{{nightly_toolchain_version}} fmt --all -- --check

update-nightly-fmt: update-nightly

update-nightly date=nightly_toolchain_version:
  rustup toolchain install --profile minimal nightly-{{ date }} --no-self-update
  rustup component add rustfmt --toolchain nightly-{{ date }}
  rustup component add clippy --toolchain nightly-{{ date }}