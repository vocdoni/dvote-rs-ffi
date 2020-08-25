# DVote Rust FFI

DVote Rust is a special Rust repository, targeted to build the libraries needed by DVote Flutter Native, to run expensive computations on mobile devices. 

They can be used with the C Foreign Function Interface on any compatible platform and architecture. 

The functions currently available are: 

```C
// Ethereum wallet
char *generate_mnemonic(int32_t size);
char *compute_private_key(const char *mnemonic_ptr, const char *hd_path_ptr);
char *compute_public_key(const char *hex_private_key_ptr);
char *compute_address(const char *hex_private_key_ptr);

// Hashing
char *digest_hex_claim(const char *hex_claim_ptr);
char *digest_string_claim(const char *str_claim_ptr);

// Signatures
char *sign_message(const char *message_ptr, const char *hex_private_key_ptr);
char *recover_signer(const char *hex_signature_ptr, const char *message_ptr);
bool is_valid(const char *hex_signature_ptr,
                        const char *message_ptr,
                        const char *hex_public_key_ptr);

char *generate_zk_proof(const char *proving_key_path, const char *inputs);

void free_cstr(char *string);
```

## Get started

- Install Rust and Cargo
- Install the Android NDK on Linux or MacOS
  - Ensure that `ANDROID_NDK_HOME` points to your NDK folder
  - On Linux, make sure that `ANDROID_NDK_HOME` targets the specific version folder, like `Android/sdk/ndk-bundle/21.1.6352462`
- Install XCode if you are targeting iOS from MacOS
- Run `make init`
- Run `make all`

### Available actions

```
$ make

 Available actions in dvote-rs:

  init       Install missing dependencies.
  
  all        Compile iOS, Android and bindings targets
  ios        Compile the iOS universal library
  android    Compile the android targets (arm64, armv7 and i686)
  bindings   Generate the .h file for iOS
  
  clean
  test

```

### Generated artifacts

Android:
- `target/aarch64-linux-android/release/libdvote.so`
- `target/armv7-linux-androideabi/release/libdvote.so`
- `target/i686-linux-android/release/libdvote.so`

iOS
- `target/universal/release/libdvote.a`

C Bindings
- `target/bindings.h`