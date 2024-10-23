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
- Use of a custom `groups` scope

## Okta implementation

- [Authorization endpoint](https://developer.okta.com/docs/api/openapi/okta-oauth/oauth/tag/OrgAS/#tag/OrgAS/operation/authorize)
- [Token endpoint](https://developer.okta.com/docs/api/openapi/okta-oauth/oauth/tag/OrgAS/#tag/OrgAS/operation/token)

### Okta differences from RFC

- The required Identity token `aud` field is renamed to `client_id` (the purpose
  and content reamins unchanged)

## Usage

```bash
$ cargo run
$ ./script/authenticate.sh
$ ./script/token.sh <code value from previous response>
```

## TODO

- Dockerfile
- Document httpie usage in scripts
- Implement error responses as per RFCs
- Logs
