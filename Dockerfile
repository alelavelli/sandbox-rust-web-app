ARG DEPLOY_ENVIRONMENT
ARG JWT_SECRET
ARG MONGODB_CONNECTION_STRING
ARG MONGODB_DB_NAME

# Build binary on separate build environment
FROM rust:1 as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

# Create final image coping the binary and adding env variables
FROM gcr.io/distroless/cc-debian12

ENV DEPLOY_ENVIRONMENT=$DEPLOY_ENVIRONMENT
ENV JWT_SECRET=$JWT_SECRET
ENV MONGODB_CONNECTION_STRING=$MONGODB_CONNECTION_STRING
ENV MONGODB_DB_NAME=$MONGODB_DB_NAME

COPY --from=build-env /app/target/release/sandbox-rust-web-app /

CMD ["./sandbox-rust-web-app"]