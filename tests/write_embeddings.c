#include <stdlib.h>

#include <finalfusion.h>

int main() {
  int status;
  ff_embeddings embeds = ff_read_embeddings("testdata/test.fifu");
  if (embeds == NULL) {
    return 1;
  }

  status = ff_write_embeddings(embeds, "testdata/write_test.fifu");

  if (status == -1) {
    return 1;
  }
  ff_free_embeddings(embeds);

  embeds = ff_read_embeddings("testdata/write_test.fifu");
  if (embeds == NULL) {
    return 1;
  }
  ff_free_embeddings(embeds);
  return 0;
}
