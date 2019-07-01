# Dockerfile for compiling & running Rust-based Command-line program	 -*-conf-*-
# that uses dictionary word lists from host computer.

# Usage on local laptop/workstation:
# docker build -f build+run.Dockerfile -t anagrams .
# docker run -it --rm -v /usr/share/dict:/usr/share/dict anagrams
# Inside that shell, run: anagrams --help

# We need to use the Rust build image, because
# we need the Rust compile and Cargo tooling
FROM rust:1.35 as build_step

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

# Prune for smaller final Docker image:
RUN strip /anagram-phrases/target/release/anagram-phrases

# Runtime image doesn't need compiler:
FROM bitnami/minideb as runtime

COPY --from=build_step /anagram-phrases/target/release/anagram-phrases /usr/local/bin/

CMD ["/bin/bash"]
