#ifndef FINALFUSION_H
#define FINALFUSION_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ff_embeddings_t *ff_embeddings;

  /**
   * Return the last error message. The pointer is valid until the
   * next finalfusion function call.
   */
  char const *ff_error(void);

  /**
   * Read finalfusion embeddings. Returns a pointer to the embeddings
   * when successful or NULL otherwise.
   */
  ff_embeddings ff_read_embeddings(char const *filename);

  /**
   * Mmap finalfusion embeddings. Returns a pointer to the embeddings
   * when successful or NULL otherwise.
   */
  ff_embeddings ff_mmap_embeddings(char const *filename);

  /**
   * Read fasttext embeddings. Returns a pointer to the embeddings
   * when successful or NULL otherwise.
   */

  ff_embeddings ff_read_fasttext(char const *filename);

  /**
   * Read embeddings in text dims format. Returns a pointer to the
   * embeddings when successful or NULL otherwise.
   */
  ff_embeddings ff_read_text_dims(char const *filename);

  /**
   * Read embeddings in text format. Returns a pointer to the
   * embeddings when successful or NULL otherwise.
   */
  ff_embeddings ff_read_text(char const *filename);

  /**
   * Read word2vec binary embeddings. Returns a pointer to the
   * embeddings when successful or NULL otherwise.
   */
  ff_embeddings ff_read_word2vec(char const *filename);

  /**
   * Write the embeddings to the given path.
   *
   * Returns -1 if writing failed, 0 on success.
   */
   int ff_write_embeddings(ff_embeddings embeddings, char const *filename);

  /**
   * Free finalfusion embeddings.
   */
  void ff_free_embeddings(ff_embeddings embeddings);

  /**
   * Returns the dimensionality of the embeddings.
   */
  size_t ff_embeddings_dims(ff_embeddings embeddings);

  /**
   * Look up the embedding for the query word and write to buffer.
   *
   * Return the embedding as a float array. If no embedding was found, NULL is returned.
   */
  float *ff_embedding_lookup(ff_embeddings embeddings, char const *word);

#ifdef __cplusplus
}
#endif

#endif /* FINALFUSION_H */
