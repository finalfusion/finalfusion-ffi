use std::mem;

use finalfusion::prelude::*;
use ndarray::Array2;

use crate::check_null;

/// Return the numer of rows in the embedding matrix.
#[no_mangle]
pub unsafe extern "C" fn ff_storage_rows(
    embeddings: *const Embeddings<VocabWrap, StorageWrap>,
) -> usize {
    check_null!(embeddings);
    let embeddings = &*embeddings;
    embeddings.storage().shape().0
}

/// Copy the entire embedding matrix.
#[no_mangle]
pub unsafe extern "C" fn ff_storage_copy(
    embeddings: *const Embeddings<VocabWrap, StorageWrap>,
) -> *mut f32 {
    check_null!(embeddings);
    let embeddings = &*embeddings;
    let array = match embeddings.storage() {
        StorageWrap::MmapArray(mmap) => mmap.view().to_owned(),
        StorageWrap::NdArray(array) => array.0.clone(),
        StorageWrap::QuantizedArray(quantized) => copy_storage_to_array(quantized.as_ref()),
        StorageWrap::MmapQuantizedArray(quantized) => copy_storage_to_array(quantized),
    };
    let mut v = array.into_raw_vec();
    let ptr = v.as_mut_ptr();
    mem::forget(v);
    ptr
}

/// Copy storage to an array.
///
/// This should only be used for storage types that do not provide
/// an ndarray view that can be copied trivially, such as quantized
/// storage.
fn copy_storage_to_array(storage: &dyn Storage) -> Array2<f32> {
    let (rows, dims) = storage.shape();

    let mut array = Array2::<f32>::zeros((rows, dims));
    for idx in 0..rows {
        array.row_mut(idx).assign(&storage.embedding(idx).as_view());
    }

    array
}
