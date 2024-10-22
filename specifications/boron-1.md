# Boron-1
## Specification: Key derivation
### Definitions: Lets assume the following
- Any kind of key will always be a 32 byte sequence.
| Symbol | Meaning            |
|--------|--------------------|
| P      | Parent key         |
| K      | Resulting key      |
| I      | Index              |
| S      | Salt               |
| \|     | Concatenate values |
### Argon2id parameters
| Name          | Value         |
| ------------- | ------------- |
| Memory (KB)   | 128 * 1024    |
| Iterations    | 4             |
| Threads       | 4             |
### Password &rarr; Master Key
With S being an optional value, it's default will be a 32 byte sequence of 0's. Password is a required UTF-8 encoded sequence. Where ARGON2 is used with P and S being paremeters returning K as the master key.
### Parent key &rarr; Child key
The function DeriveChild(P, I) &rarr; K takes a parent key P and an index I as inputs. It uses HMAC-SHA256, where P is the key and I is the message, to produce a derived child key K.
