#include <stdlib.h>
#include <string.h>

#include <finalfusion.h>

int main() {
  float *embedding;
  float *matrix_copy;
  float *embedding_from_copy;
  int i;
  size_t dims;
  ff_embeddings embeds = ff_read_embeddings("testdata/test.fifu");
  if (embeds == NULL) {
    return 1;
  }
  dims = ff_embeddings_dims(embeds);

  if (ff_storage_rows(embeds) != 41) {
    return 1;
  }

  matrix_copy = ff_storage_copy(embeds);
  if (matrix_copy == NULL) {
    return 1;
  }

  embedding_from_copy = malloc(dims * sizeof(float));
  if (embedding_from_copy) {
    memcpy(embedding_from_copy, matrix_copy, dims * sizeof(float));
  } else {
    return 1;
  }

  /* in vocab*/
  embedding = ff_embedding_lookup(embeds, "Berlin");
  if (embedding == NULL) {
    return 1;
  }
  for (i = 0; i < dims; ++i) {
    if (*(embedding + i) != *(embedding_from_copy + i)) {
      return 1;
    }
  }
  free(embedding_from_copy);
  free(embedding);
  free(matrix_copy);

  ff_free_embeddings(embeds);
  return 0;
}
