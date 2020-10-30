use dvote::encryption::symmetric;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

///////////////////////////////////////////////////////////////////////////////
// EXPORTED FUNCTIONS FUNCTIONS
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern "C" fn encrypt_symmetric(
    message_ptr: *const c_char,
    passphrase_ptr: *const c_char,
) -> *mut c_char {
    let message = unsafe { CStr::from_ptr(message_ptr) }
        .to_str()
        .expect("Invalid message string pointer");
    let passphrase = unsafe { CStr::from_ptr(passphrase_ptr) }
        .to_str()
        .expect("Invalid passphrase string pointer");

    let result = match symmetric::encrypt(message, passphrase) {
        Ok(v) => base64::encode(v),
        Err(err) => format!("ERROR: {}", err),
    };

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}

#[no_mangle]
pub extern "C" fn decrypt_symmetric(
    base64_cipher_bytes_ptr: *const c_char,
    passphrase_ptr: *const c_char,
) -> *mut c_char {
    let base64_cipher_bytes = unsafe { CStr::from_ptr(base64_cipher_bytes_ptr) }
        .to_str()
        .expect("Invalid base64_cipher_bytes string pointer");
    let passphrase = unsafe { CStr::from_ptr(passphrase_ptr) }
        .to_str()
        .expect("Invalid passphrase string pointer");

    let cipher_bytes = match base64::decode(base64_cipher_bytes) {
        Ok(v) => v,
        Err(_) => {
            return CString::new("ERROR: Invalid base64 string")
                .unwrap()
                .into_raw();
        }
    };

    let result = symmetric::decrypt(&cipher_bytes, passphrase)
        .unwrap_or_else(|err| format!("ERROR: {}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}
