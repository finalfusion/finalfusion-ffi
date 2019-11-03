use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_char, c_int};

use finalfusion::prelude::*;
use finalfusion::vocab::{Vocab, WordIndex};

use crate::check_null;

/// Indices lookup.
///
/// Returns an integer indicating how a word is represented:
///    * 0 if the word cannot be represented
///    * `n` if the word is represented by `n` embeddings
///
/// Writes the indices to `indices_res` if it is not a null-pointer.
#[no_mangle]
pub unsafe extern "C" fn ff_vocab_indices(
    embeddings: *mut Embeddings<VocabWrap, StorageWrap>,
    word: *const c_char,
    indices_res: *mut *mut usize,
) -> c_int {
    check_null!(embeddings);
    check_null!(word);
    let embeddings = &*embeddings;
    let vocab = embeddings.vocab();
    let word = CStr::from_ptr(word).to_string_lossy();
    let idx = if let Some(idx) = vocab.idx(&word) {
        idx
    } else {
        return 0;
    };
    let mut indices = match idx {
        WordIndex::Word(idx) => {
            if indices_res.is_null() {
                return 1;
            }
            vec![idx]
        }
        WordIndex::Subword(indices) => {
            if indices_res.is_null() {
                return indices.len() as c_int;
            }
            indices
        }
    };
    let len = indices.len() as c_int;
    if !indices_res.is_null() {
        let ptr = indices.as_mut_ptr();
        mem::forget(indices);
        *indices_res = ptr;
    }
    len
}

#[no_mangle]
pub unsafe extern "C" fn ff_vocab_len(
    embeddings: *mut Embeddings<VocabWrap, StorageWrap>,
) -> usize {
    check_null!(embeddings);
    let embeddings = &*embeddings;
    let vocab = embeddings.vocab();
    vocab.vocab_len()
}

#[no_mangle]
pub unsafe extern "C" fn ff_words_len(
    embeddings: *mut Embeddings<VocabWrap, StorageWrap>,
) -> usize {
    check_null!(embeddings);
    let embeddings = &*embeddings;
    let vocab = embeddings.vocab();
    vocab.words_len()
}
