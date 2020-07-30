FROM ekidd/rust-musl-builder:stable as builder

ADD . ./

RUN cargo build --release && \
    strip /home/rust/src/target/x86_64-unknown-linux-musl/release/tcp-echo-server

FROM scratch
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/tcp-echo-server /

ENTRYPOINT ["/tcp-echo-server"]
