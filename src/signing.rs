use dvote::signing;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

///////////////////////////////////////////////////////////////////////////////
// EXPORTED FUNCTIONS FUNCTIONS
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern "C" fn sign_message(
    message_ptr: *const c_char,
    hex_private_key_ptr: *const c_char,
) -> *mut c_char {
    let message = unsafe { CStr::from_ptr(message_ptr) }
        .to_str()
        .expect("Invalid message string pointer");
    let hex_private_key = unsafe { CStr::from_ptr(hex_private_key_ptr) }
        .to_str()
        .expect("Invalid hex_private_key string pointer");

    let result = signing::sign_message(message, hex_private_key)
        .unwrap_or_else(|err| format!("ERROR: {}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}

#[no_mangle]
pub extern "C" fn recover_signer(
    hex_signature_ptr: *const c_char,
    message_ptr: *const c_char,
) -> *mut c_char {
    let hex_signature = unsafe { CStr::from_ptr(hex_signature_ptr) }
        .to_str()
        .expect("Invalid hex_signature string pointer");
    let message = unsafe { CStr::from_ptr(message_ptr) }
        .to_str()
        .expect("Invalid message string pointer");

    let result = signing::recover_signer(hex_signature, message)
        .unwrap_or_else(|err| format!("ERROR: {}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}

#[no_mangle]
pub extern "C" fn is_valid(
    hex_signature_ptr: *const c_char,
    message_ptr: *const c_char,
    hex_public_key_ptr: *const c_char,
) -> bool {
    let hex_signature = unsafe { CStr::from_ptr(hex_signature_ptr) }
        .to_str()
        .expect("Invalid hex_signature string pointer");
    let message = unsafe { CStr::from_ptr(message_ptr) }
        .to_str()
        .expect("Invalid message string pointer");
    let hex_public_key = unsafe { CStr::from_ptr(hex_public_key_ptr) }
        .to_str()
        .expect("Invalid hex_public_key string pointer");

    signing::is_valid(hex_signature, message, hex_public_key)
}
