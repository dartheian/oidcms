FROM rust:alpine AS build-env
WORKDIR /usr/src/oidcms
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo install --path . --root /usr

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /usr/bin/oidcms /
CMD ["./oidcms"]

ENV AUDIENCE="api.example.com"
ENV CLIENT_SECRET="6W7XvLCrs4ByKn7Ucwh8ygeeXRhdGFdVOTp75eOc"
ENV EXPIRATION=60
ENV HOST="0.0.0.0"
ENV ISSUER="https://login.helloprima.com"
ENV PORT=4000
ENV RNG_SEED=0
ENV SECRET="c2VjcmV0"
ENV USER__ADDRESS__COUNTRY="US"
ENV USER__ADDRESS__LOCALITY="Los Angeles"
ENV USER__ADDRESS__POSTAL_CODE="90210"
ENV USER__ADDRESS__REGION="CA"
ENV USER__ADDRESS__STREET_ADDRESS="123 Hollywood Blvd."
ENV USER__EMAIL_VERIFIED="true"
ENV USER__EMAIL="john.doe@example.com"
ENV USER__FAMILY_NAME="Doe"
ENV USER__GIVEN_NAME="John"
ENV USER__LOCALE="en-US"
ENV USER__MIDDLE_NAME="James"
ENV USER__NAME="John Doe"
ENV USER__NICKNAME="Jimmy"
ENV USER__PHONE_NUMBER="+1 (425) 555-1212"
ENV USER__PROFILE="https://example.com/john.doe"
ENV USER__UPDATED_AT=946681200
ENV USER__ZONEINFO="America/Los_Angeles"
