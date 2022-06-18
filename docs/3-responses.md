# Responses
> The expected responses from all available endpoints


## ``/api/submit``
### Successful
_Nothing_

### No origin header found
- id: ``no-origin-header``

### Invalid protocol on origin header
- id: ``invalid-protocol``

### Invalid hostname on origin header
- id: ``invalid-hostname``

### Use of non-whitelisted keys on entry
- id: ``forbidden-keys``

## ``/api/retrieve``

### Successful
_Array of entries_

## ``/api/remove``
### Successful
_Nothing_

### No entry found with given duid
- id: ``not-found``

### File is not available
- id: ``not-available``

> Every endpoint also has a _catch-(almost)-all_ whose id is ``unknown``
