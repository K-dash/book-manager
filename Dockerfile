# multi stage build で、Rust のビルド環境を作成
FROM rust:1.78-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# 不要なソフトウェアは不要なので、軽量なイメージを使用
FROM debian:bookworm-slim
WORKDIR /app

# ユーザーを作成
RUN adduser book && chown -R book /app
USER book

# multi stage build でビルドしたバイナリをコピー
COPY --from=builder ./app/target/release/app ./target/release/app

# ポートを公開
ENV PORT 8080
EXPOSE $PORT
ENTRYPOINT ["./target/release/app"]
