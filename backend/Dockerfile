FROM rust:1.79

# Set the working directory inside the container
WORKDIR /project

COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the Rust application-
RUN cargo build --release



# Set the command to run the binary
CMD ["./target/release/backend"]
EXPOSE 4000