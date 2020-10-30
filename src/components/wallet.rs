use dvote::wallet;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

///////////////////////////////////////////////////////////////////////////////
// EXPORTED FUNCTIONS FUNCTIONS
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern "C" fn generate_mnemonic(size: i32) -> *mut c_char {
    let result = wallet::generate_mnemonic(size).unwrap_or_else(|err| format!("ERROR: {}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}

#[no_mangle]
pub extern "C" fn compute_private_key(
    mnemonic_ptr: *const c_char,
    hd_path_ptr: *const c_char,
) -> *mut c_char {
    let mnemonic = unsafe { CStr::from_ptr(mnemonic_ptr) }
        .to_str()
        .expect("Invalid mnemonic string pointer");
    let hd_path = unsafe { CStr::from_ptr(hd_path_ptr) }
        .to_str()
        .expect("Invalid hd_path string pointer");

    let result = wallet::compute_private_key(mnemonic, hd_path)
        .unwrap_or_else(|err| format!("ERROR: {}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}

#[no_mangle]
pub extern "C" fn compute_public_key(hex_private_key_ptr: *const c_char) -> *mut c_char {
    let hex_private_key = unsafe { CStr::from_ptr(hex_private_key_ptr) }
        .to_str()
        .expect("Invalid hex_private_key string pointer");

    let result =
        wallet::compute_public_key(hex_private_key).unwrap_or_else(|err| format!("ERROR: {}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}

#[no_mangle]
pub extern "C" fn compute_public_key_uncompressed(
    hex_private_key_ptr: *const c_char,
) -> *mut c_char {
    let hex_private_key = unsafe { CStr::from_ptr(hex_private_key_ptr) }
        .to_str()
        .expect("Invalid hex_private_key string pointer");

    let result = wallet::compute_public_key_uncompressed(hex_private_key)
        .unwrap_or_else(|err| format!("ERROR: {}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}

#[no_mangle]
pub extern "C" fn compute_address(hex_private_key_ptr: *const c_char) -> *mut c_char {
    let hex_private_key = unsafe { CStr::from_ptr(hex_private_key_ptr) }
        .to_str()
        .expect("Invalid hex_private_key string pointer");

    let result =
        wallet::compute_address(hex_private_key).unwrap_or_else(|err| format!("ERROR: {}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}
