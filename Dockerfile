FROM rust:alpine AS build-env
WORKDIR /usr/src/oidcms
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo install --path . --root /usr

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /usr/bin/oidcms /

ENV EXPIRATION=60
ENV ISSUER="http://rain.okta1.com:1802"
ENV RNG_SEED=0
ENV SECRET="c2VjcmV0"
ENV HOST="0.0.0.0"
ENV PORT=3000

HEALTHCHECK --interval=5m --timeout=3s CMD curl -f http://${HOST}:${PORT}/health || exit 1
CMD ["./oidcms"]
