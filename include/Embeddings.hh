#ifndef FINALFUSION_EMEBDDINGS_HH
#define FINALFUSION_EMEBDDINGS_HH

#include <string>
#include <vector>

#include "finalfusion.h"

class EmbeddingsImpl;

/**
 * Embeddings.
 */
class Embeddings {
public:
  /**
   * Embeddings Constructor.
   *
   * @param filename path to embeddings.
   * @param mmap memmap embeddings.
   * @throws runtime_error if Embeddings could not be read.
   */
  Embeddings(std::string const &filename, bool mmap);

  virtual ~Embeddings();

  /// Return embedding dimensionality.
  size_t dimensions();

  /**
   * Embedding lookup
   * @param word the query word
   * @return the embedding. Empty if none could be found.
   */
  std::vector<float> embedding(std::string const &word);

private:
  std::unique_ptr<EmbeddingsImpl> embeddings_impl_;
};

#endif // FINALFUSION_EMEBDDINGS_HH
