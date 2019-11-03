use std::ffi::{CStr, OsStr};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::os::raw::{c_char, c_int};
use std::os::unix::ffi::OsStrExt;
use std::{mem, ptr};

use finalfusion::io as ffio;
use finalfusion::io::WriteEmbeddings;
use finalfusion::prelude::*;
use finalfusion::storage::Storage;
use finalfusion::vocab::Vocab;

use crate::{check_null, update_error};

#[no_mangle]
pub unsafe extern "C" fn ff_read_embeddings(
    filename: *const c_char,
) -> *mut Embeddings<VocabWrap, StorageWrap> {
    check_null!(filename);
    read_embeddings_impl::<_, StorageWrap, VocabWrap>(filename, |r| Embeddings::read_embeddings(r))
}

#[no_mangle]
pub unsafe extern "C" fn ff_mmap_embeddings(
    filename: *const c_char,
) -> *mut Embeddings<VocabWrap, StorageWrap> {
    check_null!(filename);
    read_embeddings_impl::<_, StorageWrap, VocabWrap>(filename, |r| Embeddings::mmap_embeddings(r))
}

macro_rules! impl_read_non_fifu (
    ($name:ident, $method:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(
            filename: *const c_char,
        ) -> *mut Embeddings<VocabWrap, StorageWrap> {
            check_null!(filename);
            read_embeddings_impl(filename, |r| Embeddings::$method(r))
        }
    }
);

impl_read_non_fifu!(ff_read_fasttext, read_fasttext);
impl_read_non_fifu!(ff_read_word2vec, read_word2vec_binary);
impl_read_non_fifu!(ff_read_text, read_text);
impl_read_non_fifu!(ff_read_text_dims, read_text_dims);

#[no_mangle]
pub unsafe extern "C" fn ff_free_embeddings(embeddings: *mut Embeddings<VocabWrap, StorageWrap>) {
    if !embeddings.is_null() {
        Box::from_raw(embeddings);
    }
}

/// Write the embeddings.
///
/// Writes the embeddings to the given `filename`.
///
/// Returns:
///  * -1 if the file could not be created or writing failed.
///  * 0 if the file was written succesfully.
#[no_mangle]
pub unsafe extern "C" fn ff_write_embeddings(
    embeddings: *mut Embeddings<VocabWrap, StorageWrap>,
    filename: *const c_char,
) -> c_int {
    check_null!(embeddings);
    check_null!(filename);
    let embeddings = &*embeddings;
    let filename = CStr::from_ptr(filename);
    let filename = OsStr::from_bytes(filename.to_bytes());
    let mut writer = match File::create(filename) {
        Ok(f) => BufWriter::new(f),
        Err(err) => {
            update_error(&format!(
                "Could not open file {:?} for writing: {}",
                filename, err
            ));
            return -1;
        }
    };
    match embeddings.write_embeddings(&mut writer) {
        Ok(()) => 0,
        Err(err) => {
            update_error(&format!(
                "Could not write embeddings to {:?}: {}",
                filename, err
            ));
            -1
        }
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

unsafe fn read_embeddings_impl<R, S, V>(
    filename: *const c_char,
    read_embeddings: R,
) -> *mut Embeddings<VocabWrap, StorageWrap>
where
    R: FnOnce(&mut BufReader<File>) -> ffio::Result<Embeddings<V, S>>,
    S: Storage,
    V: Vocab,
    Embeddings<VocabWrap, StorageWrap>: From<Embeddings<V, S>>,
{
    let filename = CStr::from_ptr(filename);
    let filename = OsStr::from_bytes(filename.to_bytes());
    let mut reader = match File::open(filename) {
        Ok(f) => BufReader::new(f),
        Err(err) => {
            update_error(&format!("{}", err));
            return ptr::null_mut();
        }
    };
    let embeddings = match read_embeddings(&mut reader) {
        Ok(embeddings) => embeddings,
        Err(err) => {
            update_error(&format!("{}", err));
            return ptr::null_mut();
        }
    };

    // Ensure heap storage.
    let boxed_embeddings = Box::new(embeddings.into());

    Box::into_raw(boxed_embeddings)
}
