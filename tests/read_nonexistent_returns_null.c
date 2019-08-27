#include <stdio.h>

#include <finalfusion.h>

int main() {
  ff_embeddings embeds = ff_read_embeddings("foo");
  if (embeds == NULL) {
    return 0;
  }

  return 1;
}
