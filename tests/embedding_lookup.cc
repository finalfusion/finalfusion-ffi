#include <memory>
#include <vector>

#include "Embeddings.hh"
#include "finalfusion.h"

int main() {
  std::unique_ptr<Embeddings> embeddings;
  try {
    embeddings = std::make_unique<Embeddings>("foo", true);
    return 1;
  } catch (std::exception &e) {
  }

  try {
    embeddings = std::make_unique<Embeddings>("data/test.fifu", false);
  } catch (std::exception &e) {
    return 1;
  }

  std::vector<float> embedding = embeddings->embedding("Berlin");
  if (embedding.empty()) {
    return 1;
  }

  embedding = embeddings->embedding("Tübingen");
  if (!embedding.empty()) {
    return 1;
  }

  return 0;
}