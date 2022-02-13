## Authentication usage

In order to authenticate the users, Privalytics sends the user identifier (i.e. username, email...) and password to a
user-defined endpoint that should do the following:
- Be `POST`
- Accept [`LoginReq`](#LoginReq)
- Return [`EndpointResponse`](#EndpointResponse)

### Definitions
#### LoginReq

```json5
{
  "identifier": String,
  "password": String,
}
```

#### EndpointResponse

`LoginCode`: [auth/defs/responses](/src/auth/defs/responses.rs)

```json5
{
  "code": LoginCode,
  "jwt": LoginJWT,
}
```

#### LoginJWT

`LoginJWT` is a JWT using an HMAC (no Base64) secret that's `"{identifier}{password}"`

`Scopes`: [auth/defs/scopes](/src/auth/defs/scopes.rs)

The claims of the decoded JWT should be the following:
```json5
{
  "exp": number,
  "name": String,
  "scopes": Scopes[],
}
```
