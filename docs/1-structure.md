# Structure

## Saving data
- A [``bson``](https://github.com/mongodb/bson-rust) file that contains all the analytics data
```json5
// Filename is Base64-encoded Origin header

{
  "data": [
    // ...
  ]
} 
```

## Analytics structure
- The analytics data should be:
  - Anonymous by default (Pseudonymization also available)
  - Support custom data
    - ``key:value`` format
    - Where not every ``key`` sent by the client is not valid, only those defined by the server beforehand
    - Where ``value`` can be: ``String``, ``Number (u32 or u64)``, ``Bool``, ``Date`` (support for ``Tuples`` is being considered)
  - Metadata-less to preserve anonymity
    - By default, the only metadata saved is request date
    - A client-provided or server-provided unique identifier is also supported
      - Although, a UID could be used by the server, it is recommended to use a unique identifier sent by the client, that is _only_ stored on the client
    - A server-provided page is also supported

JSON Example (bare-minimum):
```json5
{
  "metadata": {
    "date": 1655023183,
    "duid": "61fd401f-5379-4cab-992a-9266477e32e1"// Identifier for this exact data entry
  },
  "data": {
    "login:success": true
  }
}
```

JSON Example (pseudonymous):
```json5
{
  "metadata": {
    "date": 1655023183,
    "page": "/app/login",
    "duid": "61fd401f-5379-4cab-992a-9266477e32e1",
    "uid": "8c026341-e36d-4700-acc3-58402e9229f2"// UID provided -and- stored by the user
  },
  "data": {
    "login:success": true
  }
}
```

JSON Example (identifiable):
```json5
{
  "metadata": {
    "date": 1655023183,
    "page": "/app/login",
    "duid": "61fd401f-5379-4cab-992a-9266477e32e1",
    "uid": "197.234.240.0"// ⚠️ Usage of account-ids, IPs, or anything that can link the data to the user is highly discouraged
  },
  "data": {
    "login:success": true,
    "login:username": "TechiePi" // ⚠️ Usage of usernames, names, etc. is also highly discouraged
  }
}
```

## Submit analytics data
There is one exposed endpoint: ``/api/submit`` that should provide a structure similar to the structure shown above:
- The client should send a _complete_ JSON instance of Analytics data (see pseudonymous and bare-minimum)
  - Some metadata is _always_ overwritten on server-side:
    - ``metadata.date``
    - ``metadata.duid``
  - Some metadata _can be_ overwritten on server-side:
    - ``metadata.page``
    - ``metadata.uid``

### Abuse prevention
The endpoint can be _partially_ protected from scrapers, bots or similar technologies:
- A ``client-key`` can be set on the ``config.toml`` file, this will enable a guard to check the required verification
data is provided by the client
  - The client needs to have the ``client-key``
  - When the client wants to send data, the following steps are also required:
    - Stringify and minify the analytics data to be sent
    - HMAC SHA256 the stringified data
    - The following header will be added to the request: ``Authorization: HMAC-SHA256 ${data}``
    - The server will run the same steps and check if both results are the same, the data is processed
- This is useful to prevent _simple and automated_ attacks, however, is someone takes the time to see how the system works,
it is quite easy to circumvent this protection. Some steps can be taken to prevent this:
  - Frequent change of the ``client-key``
    - This could be automated, but be careful to not use an API endpoint to retrieve the ``client-key`` or something like that.
    Remember: the ``client-key`` should be _constant and static_ on client-side

## Retrieve and remove analytics data
There is one endpoint to retrieve: ``/api/retrieve``.

There is one endpoint to remove: ``/api/remove``.

All endpoints require an ``Authorization`` header

### Authentication
The ``users.toml`` file contains a table with users, each user has a scope and token.
The ``Authorization`` header should look like this: ``Authorization: User ${username}:"${token}"``

#### Scopes
There are ``3`` scopes:
1. ``Read`` Can retrieve analytics data
2. ``Write`` Can retrieve and remove analytics data
3. ``Admin`` Can retrieve and remove analytics data, and retrieve internal information about the instance

### Endpoints
#### ``/api/retrieve``

**Example request body**:
```json5
{
  "from": 0,
  "to": -1 //Retrieve _all_ data
}
```

-or-

```json5
{
  "from": 0,
  "to": 50 // Retrieve first 50 entries
}
```

**Example response body**:
```json5
{
  "data": [...]
}
```

#### ``/api/remove``

**Example request body**:
```json5
{
  "remove": "61fd401f-5379-4cab-992a-9266477e32e1"// This is the ``duid``
}
```

**Example response body**:
```json5
{
  "result": 0 // Success
}
```
