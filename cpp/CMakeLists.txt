include_directories ("${PROJECT_SOURCE_DIR}/include")

add_library(embeddings Embeddings.cc)
target_link_libraries(embeddings finalfusion)