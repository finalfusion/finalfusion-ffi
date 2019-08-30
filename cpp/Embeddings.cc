#include <cstdlib>
#include <memory>
#include <stdexcept>

#include "Embeddings.hh"
#include "finalfusion.h"

void delete_ff_embeddings(ff_embeddings_t *ptr) {
  ff_free_embeddings((ff_embeddings)ptr);
}

class EmbeddingsImpl {
public:
  EmbeddingsImpl(std::string const &filename, bool mmap)
      : d_inner(nullptr, delete_ff_embeddings) {

    if (mmap) {
      d_inner.reset(ff_mmap_embeddings(filename.c_str()));
    } else {
      d_inner.reset(ff_read_embeddings(filename.c_str()));
    }

    if (d_inner == nullptr) {
      throw std::runtime_error(std::string(ff_error()));
    }
  }

  ~EmbeddingsImpl() = default;

  size_t dimensions() { return ff_embeddings_dims(d_inner.get()); }

  std::vector<float> embedding(std::string const &word) {
    float *raw_embedding = ff_embedding_lookup(d_inner.get(), word.c_str());
    if (raw_embedding == nullptr) {
      return std::vector<float>();
    }

    size_t dims = dimensions();
    std::vector<float> embedding(raw_embedding, raw_embedding + dims);
    free(raw_embedding);
    return embedding;
  }

  std::unique_ptr<ff_embeddings_t, decltype(&delete_ff_embeddings)> d_inner;
};

Embeddings::Embeddings(std::string const &filename, bool mmap)
    : embeddings_impl_(std::make_unique<EmbeddingsImpl>(filename, mmap)) {}

Embeddings::~Embeddings() = default;

std::vector<float> Embeddings::embedding(std::string const &word) {
  return embeddings_impl_->embedding(word);
}

size_t Embeddings::dimensions() { return embeddings_impl_->dimensions(); }
