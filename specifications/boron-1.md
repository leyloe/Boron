### Boron-1
### Specification: Key derivation
#### Definitions: Lets assume the following
P = parent key \
I = index \
| = concatenate values
#### Argon2id parameters
| Name          | Value         |
| ------------- | ------------- |
| Memory (KB)   | 128 * 1024    |
| Iterations    | 4             |
| Threads       | 4             |
#### Password | Salt &rarr; Master Key
todo
#### Parent key &rarr; Child key
The function DeriveChild(P, I) &rarr; 
