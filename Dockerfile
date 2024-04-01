FROM debian:bullseye

WORKDIR /app/

ADD ./target/release/simply /app/
ADD ./Rocket.toml /app/
ADD .env /app

CMD ["./simply"]