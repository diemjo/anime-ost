# RUST API
FROM rust:1.68.2 as api-builder

ENV RUSTFLAGS -C target-feature=-crt-static

RUN cargo new --bin anime-ost
WORKDIR ./anime-ost
COPY ./api/Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD ./api ./

RUN rm ./target/release/deps/anime_ost*
RUN cargo build --release

# ANGULAR WEBAPP
FROM node:hydrogen as webapp-builder

WORKDIR /app
ENV PATH /app/node_modules/.bin:$PATH

COPY ./webapp/package.json /app/package.json
RUN npm install
RUN npm install -g @angular/cli@15.2.1

COPY ./webapp /app

RUN ng build --output-path=dist

# DEPLOY
FROM nginx:1.23.4

RUN apt-get update && apt-get install -y libsqlite3-dev

COPY --from=api-builder ./anime-ost/target/release/anime-ost /usr/local/bin/anime-ost
COPY --from=webapp-builder /app/dist /usr/share/nginx/html
COPY ./nginx-anime-ost.conf /etc/nginx/conf.d/default.conf

COPY ./entrypoint.sh ./entrypoint.sh
RUN chmod +x ./entrypoint.sh

# RUN CONTAINER
ENTRYPOINT [ "./entrypoint.sh" ]