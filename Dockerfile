FROM rust:latest as app-builder

COPY . /app
WORKDIR /app

RUN cargo build --release

FROM debian:bookworm-slim

ARG USER=appuser
ARG UID=10001

RUN apt-get update \
  && apt-get install -y --no-install-recommends libssl-dev ca-certificates \
  && rm -rf /var/lib/apt/lists/* \
  && adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  "${USER}"

COPY --from=app-builder --chown=${UID}:${UID} /app/target/release/mail-forward /usr/local/bin/mail-forward
USER appuser

ENTRYPOINT ["/usr/local/bin/mail-forward"]
