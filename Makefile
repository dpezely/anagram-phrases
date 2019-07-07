# Makefile for building anagram-phrases

.PHONY: all
all: test release

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
	rustup component add rls clippy rust-analysis rust-src
	cargo install cargo-tree
	cargo install cargo-audit

.PHONY: update
update:
	rustup self update
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
	cargo build --bin anagram-phrases

# Using the help flag as a test in itself confirms the Clap config is valid
.PHONY: test
test:
	@echo "Running with --help to confirm clap config:"
	cargo run --bin anagram-phrases -- --help
	cargo test

.PHONY: audit
audit:
	cargo audit

.PHONY: release
release:
	cargo clean --release -p $(shell cargo pkgid)
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

.PHONY: clean
clean:
	cargo clean || true

.PHONY: dist-clean
dist-clean: clean
	find . -name '*~' -delete
	rm -f Cargo.lock
