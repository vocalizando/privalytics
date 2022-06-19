# Structure

## Saving data
A file is created for each entry, the name of the file is the following:

``{duid}.bson``

## Analytics structure
- The analytics data should be:
  - Anonymous by default (Pseudonymization also available)
  - Support custom data
    - ``key:value`` format
    - Where not every ``key`` sent by the client is not valid, only those defined by the server beforehand
    - Where ``value`` can be: ``String``, ``Number (i64)``, ``Bool``, ``Date`` (support for ``Tuples`` is being considered)
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
The usage of ``allowed_keys`` and ``cors_hostnames`` on the ``Config.toml`` file can help reduce abuse and can make it
more complicated to craft malicious requests.

But it is still somewhat easy to craft artificial requests and forge a ``Origin`` header. There isn't much that could be
done to prevent abuse on an open, unauthenticated API.

## Retrieve and remove analytics data
There is one endpoint to retrieve: ``/api/retrieve``.

There is one endpoint to delete: ``/api/delete``.

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
  "duid": "61fd401f-5379-4cab-992a-9266477e32e1"
}
```

**Example response body**:
```json5
{
  "result": 0 // Success
}
```
