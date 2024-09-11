# Makefile for building anagram-phrases

.PHONY: all
all: test clippy format release

CARGO_BIN ?= /home/${USER}/.cargo/bin

PATH := "${CARGO_BIN}:${PATH}"

CRATE_DIR=$(shell ${CARGO_BIN}/cargo locate-project | jq .root | sed 's%/Cargo.toml%%')

# If developing for this project:
.PHONY: developer
developer: deps

# This is done for you within Docker `rust` images for server instances.
# If you are deploying to a fresh workstation/laptop, start here.
# It is essentially https://www.rust-lang.org/en-US/install.html
# but modified with `-y` flag for full automation, plus some goodies.
.PHONY: deps
deps:
	curl -sS --output /tmp/install-rust.sh https://sh.rustup.rs
	/bin/bash /tmp/install-rust.sh -y
	PATH=${PATH} \
	  rustup component add clippy rust-analyzer rust-src rustfmt
	PATH=${PATH} \
	  rustup +nightly component add clippy
	PATH=${PATH} \
	  cargo install cargo-tree
	PATH=${PATH} \
	  cargo install cargo-audit

.PHONY: update
update:
	PATH=${PATH} \
	  rustup self update
	PATH=${PATH} \
	  rustup update stable

# By default the Rust installation modifies PATH within ~/.profile
# but you may want this set within ~/.bashrc instead:
.PHONY: dot-bashrc
dot-bashrc:
	grep -c '\.cargo/bin' ~/.bashrc && \
	  (echo '~/.bashrc already configured' && false) || true
	@echo 'PATH="$${PATH}:$${HOME}/.cargo/bin"' >> ~/.bashrc
	@echo 'if [ $$(which rustc) ]; then'  >> ~/.bashrc
	@echo -n '   export RUST_SRC_PATH=' >> ~/.bashrc
	@echo '"$$(rustc --print sysroot)/lib/rustlib/src/rust/src"' >> ~/.bashrc
	@echo '   export RUST_BACKTRACE=1' >> ~/.bashrc
	@echo 'fi' >> ~/.bashrc
	@echo "Next, manually run: . ~/.bashrc"

.PHONY: build
build:
	PATH=${PATH} \
	  cargo build --bin anagram-phrases

# Using the help flag as a test in itself confirms the Clap config is valid
.PHONY: test
test:
	@echo "Running with --help to confirm clap config:"
	[ $(shell PATH=${PATH} \
	  cargo run --bin anagram-phrases -- --help | wc -l) = 29 ]
	PATH=${PATH} \
	  cargo test

.PHONY: clippy
clippy:
	PATH=${PATH} \
	  cargo clippy --no-deps --all-targets --all-features -- -D warnings

.PHONY: format
format:
	PATH=${PATH} \
	  cargo fmt --check

.PHONY: release
release:
	[ -f Cargo.lock ] && PATH=${PATH} cargo clean --release -p anagram-phrases || true
	PATH=${PATH} \
	  cargo build --release --bin anagram-phrases

# Build release for production using Docker with a Linux base image.
# (Avoid cross-compiling on macOS with Linux target, which is
# problematic due to third-party toolchains like crosstool-ng
# abandoning macOS as of 2018-11-26.)
# For most use cases, you only need the minimal "Docker Desktop for Mac".
# https://docs.docker.com/docker-for-mac/docker-toolbox/
# See also .dockerignore file.
.PHONY: production-release
production-release:
	@which docker || \
	  echo 'Install: https://download.docker.com/mac/stable/Docker.dmg'
	docker build -f build-only.Dockerfile -t anagram-phrases-build .
	[ -d target ] || \
	  mkdir -p target
	docker run -it --rm \
	  -v "$(shell pwd)/target:/var/tmp/bin" \
	  anagram-phrases-build \
	  cp -p /anagram-phrases/target/release/anagram-phrases \
	        /var/tmp/bin/anagram-phrases.x86_64-unknown-linux-gnu
	ls -lh target/anagram*

future:
	RUSTFLAGS="-D warnings" \
	PATH=${PATH} \
	  cargo build --release --future-incompat-report

# Ensure working directory contains Cargo.lock
# because cargo-audit gets lost if started in ./src/ or lower.
.PHONY: audit
audit:
	cd ${CRATE_DIR} && \
	RUSTFLAGS="-D warnings" \
	PATH=${PATH} \
	  cargo audit

.PHONY: clean
clean:
	cargo clean || true

.PHONY: dist-clean
dist-clean: clean
	git clean -dXff
