# DVote Rust FFI

### 0.6.2

- Adding support for symmetric encryption using SecretBox (libsodium)

### 0.6.1

- Supporting signatures with `v` values of `0x1b-0x1c` and `0x00-0x01`

### 0.6.0

- Adding support for compressed public keys
- **Breaking**: By default `compute_public_key` now returns compressed public keys
- **Breaking**: `recover_signer` also returns compressed public keys

### 0.5.0

- Using the refactored dependencies from dvote-rs
