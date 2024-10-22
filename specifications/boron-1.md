# Boron-1
## Specification: Key derivation
### Definitions: Lets assume the following
| Symbol | Meaning            |
|--------|--------------------|
| P      | Parent key         |
| K      | key                |
| I      | Index              |
| \|     | Concatenate values |
### Argon2id parameters
| Name          | Value         |
| ------------- | ------------- |
| Memory (KB)   | 128 * 1024    |
| Iterations    | 4             |
| Threads       | 4             |
### Password | Salt &rarr; Master Key
todo
### Parent key &rarr; Child key
The function defined as DeriveChild(P, I) &rarr; K where K is derived from Hmac-Sha256 with P as the key and I as the message.

The function DeriveChild(P, I) &rarr; K takes a parent key P and an index I as inputs. It uses HMAC-SHA256, where P is the key and I is the message, to produce a derived child key K.