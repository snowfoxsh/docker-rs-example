FROM rust:latest as builder

WORKDIR /code
COPY . .
RUN ["cargo", "build", "--release"]

FROM ubuntu

COPY --from=builder /code/target/release/docker-rs exe
ENTRYPOINT ["/exe"]
EXPOSE 8080