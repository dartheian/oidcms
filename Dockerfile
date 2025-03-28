FROM rust:alpine
WORKDIR /usr/src/oidcms
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo install --path .

ENV AUDIENCE="api.example.com"
ENV CLIENT_SECRET="6W7XvLCrs4ByKn7Ucwh8ygeeXRhdGFdVOTp75eOc"
ENV EXPIRATION=60
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
ENV USER__GROUPS="group1,group2,group3"
ENV USER__GIVEN_NAME="John"
ENV USER__LOCALE="en-US"
ENV USER__MIDDLE_NAME="James"
ENV USER__NAME="John Doe"
ENV USER__NICKNAME="Jimmy"
ENV USER__PERSONIO_EMAIL="john.doe@personio.com"
ENV USER__PHONE_NUMBER="+1 (425) 555-1212"
ENV USER__PREFERRED_USERNAME="johnny"
ENV USER__PROFILE="https://example.com/john.doe"
ENV USER__UPDATED_AT=946681200
ENV USER__ZONEINFO="America/Los_Angeles"

CMD ["oidcms"]
HEALTHCHECK --interval=1m --timeout=2s --start-period=5s --start-interval=1s --retries=3 CMD wget --no-verbose --tries=1 --spider http://${HOST}:${PORT}/health
