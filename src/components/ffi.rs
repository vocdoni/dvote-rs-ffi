use std::ffi::CString;
use std::os::raw::c_char;

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
