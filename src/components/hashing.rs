use dvote::hashing;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

///////////////////////////////////////////////////////////////////////////////
// EXPORTED FUNCTIONS FUNCTIONS
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern "C" fn digest_string_claim(str_claim_ptr: *const c_char) -> *mut c_char {
    let str_claim = unsafe { CStr::from_ptr(str_claim_ptr) }
        .to_str()
        .expect("Invalid str_claim string pointer");

    let result =
        hashing::digest_string_claim(str_claim).unwrap_or_else(|err| format!("ERROR: {}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}

#[no_mangle]
pub extern "C" fn digest_hex_claim(hex_claim_ptr: *const c_char) -> *mut c_char {
    let hex_claim = unsafe { CStr::from_ptr(hex_claim_ptr) }
        .to_str()
        .expect("Invalid hex_claim string pointer");

    let result =
        hashing::digest_hex_claim(hex_claim).unwrap_or_else(|err| format!("ERROR: {}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}
