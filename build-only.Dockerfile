# Dockerfile for compiling Rust-based Command-line program	 -*-conf-*-

# We need to use the Rust build image, because
# we need the Rust compile and Cargo tooling
FROM rust:1.35

# Create dummy project used only for dependencies
RUN USER=root cargo new --bin anagram-phrases
WORKDIR /anagram-phrases

# Copy *only* manifest files, for making an image with just dependencies
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build dependencies
RUN cargo build --release --bin anagram-phrases

# Remove fake source code from dummy project
RUN rm src/*.rs
RUN rm /anagram-phrases/target/release/anagram-phrases

# Copy only the actual source code to avoid invalidating the cache
COPY ./src/ ./src/
# Ensure that the build process sees our actual source code as newer
# than previously compiled dummy version:
RUN touch src/*.rs

# Build again using actual source files intended for production:
RUN cargo build --release --bin anagram-phrases

# Final steps after building this container to deploy the new executable:
#
# docker run -it --rm \
#	 -v "$(shell pwd)/target/linux:/var/tmp/bin" \
#	 anagram-phrases-build \
#	  cp -p /anagram-phrases/target/release/anagram-phrases \
#	        /var/tmp/bin/anagram-phrases.x86_64-unknown-linux-gnu
# ls -lh target/linux/ana*
# -rwxr-xr-x  1 djp  staff  3.5M 25 Jun 12:34 target/linux/anagram-phrases.x86_64-unknown-linux-gnu
