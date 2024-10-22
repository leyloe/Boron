### Boron-1
### Specification: Key derivation
#### Argon2id parameters
| Name          | Value         |
| ------------- | ------------- |
| Memory (KB)   | 128 * 1024    |
| Iterations    | 4             |
| Threads       | 4             |
#### Definitions: Lets assume the following
P = parent key \
I = index \
| = concatenate values
#### Password | Salt &rarr; Master Key
todo
#### Parent key &rarr; Child key
The function DeriveChild(P, I) &rarr; 
