use std::cell::RefCell;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::os::raw::c_char;
use std::os::unix::ffi::OsStrExt;
use std::ptr;

use finalfusion::prelude::*;

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

#[no_mangle]
pub extern "C" fn ff_read_embeddings(
    filename: *const c_char,
) -> *mut Embeddings<VocabWrap, StorageWrap> {
    assert!(!filename.is_null(), "filename was a NULL pointer");

    let filename = unsafe { CStr::from_ptr(filename) };
    let filename = OsStr::from_bytes(filename.to_bytes());

    let mut reader = match File::open(filename) {
        Ok(f) => BufReader::new(f),
        Err(err) => {
            update_error(&format!("{}", err));
            return ptr::null_mut();
        }
    };

    let embeddings = match Embeddings::read_embeddings(&mut reader) {
        Ok(embeddings) => embeddings,
        Err(err) => {
            update_error(&format!("{}", err));
            return ptr::null_mut();
        }
    };

    // Ensure heap storage.
    let boxed_embeddings = Box::new(embeddings);

    Box::into_raw(boxed_embeddings)
}

#[no_mangle]
pub extern "C" fn ff_free_embeddings(embeddings: *mut Embeddings<VocabWrap, StorageWrap>) {
    if !embeddings.is_null() {
        unsafe {
            Box::from_raw(embeddings);
        }
    }
}
