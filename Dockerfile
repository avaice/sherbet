FROM rust:1.67

WORKDIR /usr/src/sherbet

# 3. 必要なファイルをコピー
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY public ./public
COPY .env.example ./.env

RUN cargo install --path .


EXPOSE 8080

CMD ["sherbet"]
