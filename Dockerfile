FROM rust:1.76 as builder
WORKDIR /usr/src/ymnab
COPY . .
RUN cargo install --path .

FROM alpine:3.19
COPY --from=builder /usr/local/cargo/bin/ymnab /usr/local/bin/ymnab
CMD ["ymnab"]
