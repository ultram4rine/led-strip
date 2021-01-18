FROM rust:1.49 as builder

RUN USER=root cargo new --bin led-strip
WORKDIR ./led-strip
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/led_strip*
RUN cargo build --release

FROM node:15 as frontend

WORKDIR ./ui
COPY ./ui/package.json ./ui/package-lock.json ./
RUN npm install

COPY . .
RUN npm run build

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 3030

ENV TZ=Europe/Saratov \
    APP_USER=led

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /led-strip/target/release/led-strip ${APP}/led-strip
COPY --from=frontend /ui/public/build ${APP}/ui/public

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./led-strip"]