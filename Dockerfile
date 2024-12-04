FROM rust:alpine AS rust-build

ADD . ./app_build

RUN apk --update add git build-base && \
    cd /app_build ; cargo build --release

FROM alpine:3.20

WORKDIR /tw

COPY --from=rust-build /app_build/target/release/ddstats-cli /tw/ddstats-cli

CMD ["/tw/ddstats-cli", "master-parser"]
