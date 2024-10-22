# Boron-1
## Specification: Key derivation
### Definitions: Lets assume the following
- Any kind of key will always be a 32-byte sequence.

| Symbol | Meaning            |
|--------|--------------------|
| P      | Parent key         |
| K      | Resulting key      |
| I      | Index              |
| S      | Salt               |
| L      | Label              |
| U      | Username           |
| E      | Resulting entry    |
| \|     | Concatenate values |
### Argon2id parameters
| Name          | Value         |
| ------------- | ------------- |
| Memory (KB)   | 128 * 1024    |
| Iterations    | 4             |
| Threads       | 4             |
### Password &rarr; Master Key
With S being an optional value, it's default will be a 32-byte sequence of 0's. Password is a required UTF-8 encoded sequence. Where ARGON2 is used with (P) and (S) being function arguments returning (K) as the master key.
### Parent key &rarr; Child key
The function DeriveChild(P, I) &rarr; K takes a parent key (P) and an index (I) as function arguments, returning a key (K). It uses HMAC-SHA256, where (P) is the key and (I) is the message, to produce a derived child key (K). (I) is 32 unsigned and big endian encoded.
### Key &rarr; Entry
The function GetEntry(K, L, U) -> K takes a resulting key (K), a label (L), and a username (U) as function arguments, returning a new key (E). It uses HMAC-SHA256, where (K) is the key and (L | U) is the message, to produce a key (E).
### The key tree
This part is simple, as an example just do DeriveChild(DeriveChild(DeriveChild(P,3),2),5) for P/3/2/5 or likewise for your desired path.