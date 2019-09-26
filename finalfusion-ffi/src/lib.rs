pub mod embeddings;
pub mod vocab;

use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;

#[macro_export]
macro_rules! check_null (
 ($ptr:expr) => {
    assert!(!$ptr.is_null(), "{} was a NULL pointer.", stringify!($ptr));
 }
);

thread_local! {
    static ERROR: RefCell<CString> = RefCell::new(CString::new(Vec::new()).unwrap());
}

fn update_error(err: &str) {
    ERROR
        .with(|e| e.replace(CString::new(err.as_bytes()).expect("Error string contains nul byte")));
}

/// Return the last error message.
///
/// The returned pointer is owned by the library and is valid until
/// the next function call that can result in an error.
#[no_mangle]
pub extern "C" fn ff_error() -> *const c_char {
    ERROR.with(|f| f.borrow().as_ptr())
}
