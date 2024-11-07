FROM rust:alpine AS build-env
WORKDIR /usr/src/oidcms
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo install --path . --root /usr

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /usr/bin/oidcms /
COPY --from=build-env /usr/bin/oidcms /user.json

ENV CLIENT_URI="http://localhost"
ENV EXPIRATION=60
ENV HOST="0.0.0.0"
ENV ISSUER="http://rain.okta1.com:1802"
ENV PORT=3000
ENV RNG_SEED=0
ENV SECRET="c2VjcmV0"
ENV USER_FILE="user.json"

HEALTHCHECK --interval=5m --timeout=3s CMD curl -f http://${HOST}:${PORT}/health || exit 1
CMD ["./oidcms"]
