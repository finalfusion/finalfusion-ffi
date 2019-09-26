#include <stdlib.h>

#include <finalfusion.h>

int main() {
  int n;
  size_t* indices;

  ff_embeddings embeds = ff_read_embeddings("testdata/test.fifu");
  if (embeds == NULL) {
    return 1;
  }

  n = ff_vocab_indices(embeds, "Berlin", &indices);

  if (n != 1) {
    return 1;
  }

  if (*indices != 0) {
    return 1;
  }
  free(indices);

  if (ff_vocab_len(embeds) != 41) {
    return 1;
  }

  if (ff_words_len(embeds) != 41) {
    return 1;
  }


  ff_free_embeddings(embeds);

  return 0;
}
