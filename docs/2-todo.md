# To Do

> The implementation of privalytics is not yet complete. Here's a list with what's missing

- [ ] Entry Structure
  - [ ] Data
    - [x] ``key:value`` format
    - [x] Key whitelisting
    - [ ] Value support
      - [x] String
      - [x] Number (i64)
      - [x] Bool
      - [ ] Date
      - [ ] Tuples
  - [ ] Metadata
    - [x] Request date (server-side)
    - [ ] UID
      - [x] Client provided
      - [ ] Server provided
    - [ ] Page
      - [x] Client provided
      - [ ] Server provided
- [ ] API
  - [ ] ``/api/submit``
    - [x] Restriction-less submit
    - [ ] ``client-key``-protected submit
    - [ ] JSON response
  - [ ] ``/api/retrieve``
    - [x] Retrieve data
    - [ ] Fine grain control of data received
    - [ ] JSON response
  - [ ] ``/api/remove``
    - [x] Remove entry from given ``duid``
    - [ ] JSON response
