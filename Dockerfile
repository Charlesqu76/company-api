FROM rust:1.74.0 as builder

RUN USER=root cargo new --bin company-api
WORKDIR ./company-api
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release


# Build web app with own code
RUN rm src/*.rs
ADD . ./
RUN cargo build --release


FROM debian:12.4
ARG APP=/usr/src/app


EXPOSE 3002

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /company-api/target/release/company-api ${APP}/company-api

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./company-api"]