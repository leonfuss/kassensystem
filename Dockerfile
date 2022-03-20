FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 AS chef

# Switch to working directory - create if it does non exist
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
# Copy all files from working directory to Docker image
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/reciepe.json recipe.json
# Build project dependencies not project itself
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if dependencies stay the same,
# all layers should be cached
COPY . .
# set offline flag for sqlx querry comilation
ENV SQLX_OFFLINE true
# Build Binary
RUN cargo build --release --bin ausgleichende_gerechtigkeit


FROM debina:bullseye-slim as release
WORKDIR /app

RUN apt-get update -y \
    ## some dependencies depend on openssl && ca-certificates is needed for TSL
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # clean update
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
# only copy compiled binary
COPY --from=builder /app/target/release/ausgleichende_gerechtigkeit ausgleichende_gerechtigkeit
# need config files at runtime
COPY config config
ENV APP_ENVIROMENT production
ENTRYPOINT ["./ausgleichende_gerechtigkeit"]
