#include <stdlib.h>

#include <finalfusion.h>

int main() {
  float *embedding;
  ff_embeddings embeds = ff_read_text("testdata/test.txt");
  if (embeds == NULL) {
    return 1;
  }

  /* in vocab*/
  embedding = ff_embedding_lookup(embeds, "Berlin");
  if (embedding == NULL) {
    return 1;
  }
  free(embedding);

  /* oov */
  embedding = ff_embedding_lookup(embeds, "Tübingen");
  if (embedding != NULL) {
    return 1;
  }
  free(embedding);

  ff_free_embeddings(embeds);
  return 0;
}
