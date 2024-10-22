# Boron-1 Specification: Key Derivation
### Key Definitions
All keys will be 32-byte sequences, ensuring uniformity across the system.

| Symbol | Meaning                        |
|--------|--------------------------------|
| P      | Parent key                     |
| K      | Resulting (derived) key        |
| I      | Index (32-bit unsigned integer)|
| S      | Salt (optional, 32 bytes)      |
| L      | Label                          |
| U      | Username (optional)            |
| \|     | Concatenate                    |

### Argon2id Parameters

| Parameter    | Value         |
|--------------|---------------|
| Memory (KB)  | 128 * 1024    |
| Iterations   | 4             |
| Threads      | 4             |

### Master Key Derivation (Password &rarr; Master Key)
1. **Inputs:**
   - **Password**: Required, UTF-8 encoded.
   - **Salt (S)**: Optional, default is a 32-byte sequence of zeros.
2. **Process**: Argon2id is used for deriving the master key (K). The password (P) and salt (S) are passed as inputs to the Argon2id function.
3. **Output**: The result is a 32-byte master key (K).

### Child Key Derivation (Parent Key &rarr; Child Key)
1. **Inputs:**
   - **Parent Key (P)**: The key from which to derive a child key.
   - **Index (I)**: A 32-bit unsigned integer, big-endian encoded.
2. **Process**: HMAC-SHA256 is used to derive a child key. The parent key (P) is the HMAC key, and the index (I) is the message.
3. **Output**: The resulting key (K) is a 32-byte child key.

### Entry Derivation (Key &rarr; Entry)
1. **Inputs:**
   - **Parent Key (P)**: The base key.
   - **Label (L)**: An identifier or label.
   - **Username (U)**: Optional, can be an empty string ("").
2. **Process**: HMAC-SHA256 is used, with the parent key (P) as the HMAC key and the concatenation of the label and username (L | U) as the message.
3. **Output**: The resulting 32-byte key (K).

### Key Derivation Paths
The key derivation system supports hierarchical key derivation using paths. 
For example:
- **Simple path**: `DeriveChild(DeriveChild(DeriveChild(P, 3), 2), 5)` produces a key for the path `P/3/2/5`.
- **Path with an entry**: `DeriveChild(DeriveChild(GetEntry(P, L, U), 2), 5)` produces a key for the path `P/labelusername/2/5`.

This structure provides flexibility for generating unique keys across various use cases, leveraging Argon2id for secure master key generation and HMAC-SHA256 for efficient child and entry key derivation.
