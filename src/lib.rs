extern crate za_prover;

use dvote::hashing;
use dvote::signing;
use dvote::wallet;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use za_prover::groth16;
use za_prover::groth16::helper;

///////////////////////////////////////////////////////////////////////////////
// EXPORTED FUNCTIONS FUNCTIONS
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern "C" fn digest_string_claim(str_claim_ptr: *const c_char) -> *mut c_char {
    let str_claim = unsafe { CStr::from_ptr(str_claim_ptr) }
        .to_str()
        .expect("Invalid str_claim string");

    let result =
        hashing::digest_string_claim(str_claim).unwrap_or_else(|err| format!("ERROR: {}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}

#[no_mangle]
pub extern "C" fn digest_hex_claim(hex_claim_ptr: *const c_char) -> *mut c_char {
    let hex_claim = unsafe { CStr::from_ptr(hex_claim_ptr) }
        .to_str()
        .expect("Invalid hex_claim string");

    let result =
        hashing::digest_hex_claim(hex_claim).unwrap_or_else(|err| format!("ERROR: {}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}

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
        .expect("Invalid mnemonic string");
    let hd_path = unsafe { CStr::from_ptr(hd_path_ptr) }
        .to_str()
        .expect("Invalid hd_path string");

    let result = wallet::compute_private_key(mnemonic, hd_path)
        .unwrap_or_else(|err| format!("ERROR: {}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}

#[no_mangle]
pub extern "C" fn compute_public_key(hex_private_key_ptr: *const c_char) -> *mut c_char {
    let hex_private_key = unsafe { CStr::from_ptr(hex_private_key_ptr) }
        .to_str()
        .expect("Invalid hex_private_key string");

    let result =
        wallet::compute_public_key(hex_private_key).unwrap_or_else(|err| format!("ERROR: {}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}

#[no_mangle]
pub extern "C" fn compute_address(hex_private_key_ptr: *const c_char) -> *mut c_char {
    let hex_private_key = unsafe { CStr::from_ptr(hex_private_key_ptr) }
        .to_str()
        .expect("Invalid hex_private_key string");

    let result =
        wallet::compute_address(hex_private_key).unwrap_or_else(|err| format!("ERROR: {}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}

#[no_mangle]
pub extern "C" fn sign_message(
    message_ptr: *const c_char,
    hex_private_key_ptr: *const c_char,
) -> *mut c_char {
    let message = unsafe { CStr::from_ptr(message_ptr) }
        .to_str()
        .expect("Invalid message string");
    let hex_private_key = unsafe { CStr::from_ptr(hex_private_key_ptr) }
        .to_str()
        .expect("Invalid hex_private_key string");

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
        .expect("Invalid hex_signature string");
    let message = unsafe { CStr::from_ptr(message_ptr) }
        .to_str()
        .expect("Invalid message string");

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
        .expect("Invalid hex_signature string");
    let message = unsafe { CStr::from_ptr(message_ptr) }
        .to_str()
        .expect("Invalid message string");
    let hex_public_key = unsafe { CStr::from_ptr(hex_public_key_ptr) }
        .to_str()
        .expect("Invalid hex_public_key string");

    signing::is_valid(hex_signature, message, hex_public_key)
}

#[no_mangle]
pub extern "C" fn generate_zk_proof(
    proving_key_path: *const c_char,
    inputs: *const c_char,
) -> *mut c_char {
    let proving_key_path = unsafe { CStr::from_ptr(proving_key_path) };
    let proving_key_path = proving_key_path
        .to_str()
        .expect("Could not parse proving_key_path");

    let inputs = unsafe { CStr::from_ptr(inputs) };
    let inputs = inputs.to_str().expect("Could not parse the inputs");

    let result = groth16::flatten_json("main", &inputs)
        .and_then(|inputs| helper::prove(&proving_key_path, inputs))
        .unwrap_or_else(|err| format!("ERROR: {:?}", err));

    CString::new(result).unwrap().into_raw()

    // NOTE: Caller must free() the resulting pointer
}

///////////////////////////////////////////////////////////////////////////////
// STRING FREE
///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern "C" fn free_cstr(string: *mut c_char) {
    unsafe {
        if string.is_null() {
            return;
        }
        CString::from_raw(string)
    };
}
