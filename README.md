# OIDC mock server

A mock implementation of an OIDC server that authenticate users using the
Authorization Code Flow. The mandatory presence of some optional parameters and
the Form Post Response Mode are opinionated. The renaming of some required
standard fields follows Okta specifications. Only the required fields of the
Authentication and the Identity tokens are included in the responses.

![OIDC Authorization Code Flow with PKCE](https://www.mermaidchart.com/raw/d4687c7d-49cc-44ad-86c4-668490ea6eca?theme=light&version=v0.1&format=svg)

## References

- [The OAuth 2.0 Authorization Framework](https://datatracker.ietf.org/doc/html/rfc6749)
- [OpenID Connect Core 1.0](https://openid.net/specs/openid-connect-core-1_0.html)
- [Proof Key for Code Exchange by OAuth Public Clients](https://datatracker.ietf.org/doc/html/rfc7636)
- [Form Post Response mode](https://openid.net/specs/oauth-v2-form-post-response-mode-1_0.html)
- [OAuth 2.0 Threat Model and Security Considerations](https://datatracker.ietf.org/doc/html/rfc6819)
- [Json Web Token](https://datatracker.ietf.org/doc/html/rfc7519)

### Implementation differences from RFC

- The errors are unstructured human-readable strings, useful for debugging
  purposes

## Okta implementation

- [Tokens](https://developer.okta.com/docs/api/openapi/okta-oauth/guides/overview/)
- [Authorization endpoint](https://developer.okta.com/docs/api/openapi/okta-oauth/oauth/tag/OrgAS/#tag/OrgAS/operation/authorize)
- [Token endpoint](https://developer.okta.com/docs/api/openapi/okta-oauth/oauth/tag/OrgAS/#tag/OrgAS/operation/token)
- [Userinfo endpoint](https://developer.okta.com/docs/api/openapi/okta-oauth/oauth/tag/OrgAS/#tag/OrgAS/operation/userinfo)

### Extensions

- Okta uses a custom `groups` scope

## Usage

The simplest way is to use the provided Dockerfile:

```toml
services:
  idp:
    image: ghcr.io/dartheian/oidcms:latest
    ports:
      - 4000:4000
```

The `script` directory contains some shell scripts that show the interactions with the server. To run them you need [`httpie`](https://httpie.io) installed in your path and overwrite any value in the scripts that does not match the default ones provided by the container.

```bash
cargo run
./script/health.sh
./script/authenticate.sh
./script/token.sh <code value from previous response>
./script/userinfo.sh <access token value from previous response>
```

## Configuration

The configuration is loaded from the environment and can be overwritten using docker compose (e.g. using the `environment` or the `env_file` attributes)

```toml
services:
  idp:
    image: ghcr.io/dartheian/oidcms:latest
    environment:
      - PORT=5000
    expose:
      - 5000
```

### Server

- `AUDIENCE`: The audience (`aud`) claim value to put in the tokens (defaults to `api.example.com`)
- `CLIENT_SECRET`: The client secret to access protected endpoints such as `/token` (defaults to `6W7XvLCrs4ByKn7Ucwh8ygeeXRhdGFdVOTp75eOc`)
- `EXPIRATION`: The expiration time of the tokens expressed in seconds (defaults to `60`)
- `HOST`: The host of the oidcms server (defaults to `0.0.0.0`)
- `ISSUER`: The issuer (`iss`) claim value to put in the tokens (defaults to `https://login.helloprima.com`)
- `PORT`: The port of the oidcms server (defaults to `4000`)
- `RNG_SEED`: The seed of the pseudorandom number generator (defaults to `0`)
- `SECRET`: The secret key used to sign the tokens (defaults to `c2VjcmV0`)

### User info

- `USER__ADDRESS__COUNTRY`: defaults to `US`
- `USER__ADDRESS__LOCALITY`: defaults to `Los Angeles`
- `USER__ADDRESS__POSTAL_CODE`: defaults to `90210`
- `USER__ADDRESS__REGION`: defaults to `CA`
- `USER__ADDRESS__STREET_ADDRESS`: defaults to `123 Hollywood Blvd.`
- `USER__EMAIL_VERIFIED`: defaults to `true`
- `USER__EMAIL`: defaults to `john.doe@example.com`
- `USER__FAMILY_NAME`: defaults to `Doe`
- `USER__GIVEN_NAME`: defaults to `John`
- `USER__LOCALE`: defaults to `en-US`
- `USER__MIDDLE_NAME`: defaults to `James`
- `USER__NAME`: defaults to `John Doe`
- `USER__NICKNAME`: defaults to `Jimmy`
- `USER__PHONE_NUMBER`: defaults to `+1 (425) 555-1212`
- `USER__PROFILE`: defaults to `https://example.com/john.doe`
- `USER__UPDATED_AT`: defaults to `946681200` (Thu Jul 21 2011 20:42:50 GMT+0000)
- `USER__ZONEINFO`: defaults to `America/Los_Angeles`

## TODO

- Implement error responses as per RFCs
- Logs
