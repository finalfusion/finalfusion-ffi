#ifndef FINALFUSION_H
#define FINALFUSION_H

#include <stddef.h>

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
   * Free finalfusion embeddings.
   */
  void ff_free_embeddings(ff_embeddings embeddings);

#ifdef __cplusplus
}
#endif

#endif // FINALFUSION_H
