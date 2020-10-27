extern crate za_prover;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use za_prover::groth16;
use za_prover::groth16::helper;

///////////////////////////////////////////////////////////////////////////////
// EXPORTED FUNCTIONS FUNCTIONS
///////////////////////////////////////////////////////////////////////////////

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
