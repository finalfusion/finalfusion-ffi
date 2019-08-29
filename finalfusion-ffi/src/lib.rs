use std::cell::RefCell;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::os::raw::c_char;
use std::os::unix::ffi::OsStrExt;
use std::{mem, ptr};

use finalfusion::prelude::*;

macro_rules! check_null(
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

#[no_mangle]
pub unsafe extern "C" fn ff_read_embeddings(
    filename: *const c_char,
) -> *mut Embeddings<VocabWrap, StorageWrap> {
    check_null!(filename);
    read_embeddings(filename, false)
}

#[no_mangle]
pub unsafe extern "C" fn ff_mmap_embeddings(
    filename: *const c_char,
) -> *mut Embeddings<VocabWrap, StorageWrap> {
    check_null!(filename);
    read_embeddings(filename, true)
}

#[no_mangle]
pub unsafe extern "C" fn ff_free_embeddings(embeddings: *mut Embeddings<VocabWrap, StorageWrap>) {
    if !embeddings.is_null() {
        Box::from_raw(embeddings);
    }
}

/// Return the embedding dimensionality.
#[no_mangle]
pub unsafe extern "C" fn ff_embeddings_dims(
    embeddings: *const Embeddings<VocabWrap, StorageWrap>,
) -> usize {
    check_null!(embeddings);
    let embeddings = &*embeddings;
    embeddings.storage().shape().1
}

/// Embedding Lookup.
///
/// Return the embedding as a float array. If no embedding was found, NULL is returned.
#[no_mangle]
pub unsafe extern "C" fn ff_embedding_lookup(
    embeddings: *const Embeddings<VocabWrap, StorageWrap>,
    word: *const c_char,
) -> *mut f32 {
    check_null!(embeddings);
    check_null!(word);

    let embeddings = &*embeddings;
    let word = CStr::from_ptr(word).to_string_lossy();
    if let Some(embedding) = embeddings.embedding(&word) {
        let mut embedding = embedding.into_owned().into_raw_vec();
        let ptr = embedding.as_mut_ptr();
        mem::forget(embedding);
        return ptr;
    }
    ptr::null_mut()
}

unsafe fn read_embeddings(
    filename: *const c_char,
    mmap: bool,
) -> *mut Embeddings<VocabWrap, StorageWrap> {
    let filename = CStr::from_ptr(filename);
    let filename = OsStr::from_bytes(filename.to_bytes());
    let mut reader = match File::open(filename) {
        Ok(f) => BufReader::new(f),
        Err(err) => {
            update_error(&format!("{}", err));
            return ptr::null_mut();
        }
    };
    let embeddings = if mmap {
        Embeddings::mmap_embeddings(&mut reader)
    } else {
        Embeddings::read_embeddings(&mut reader)
    };

    let embeddings = match embeddings {
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
