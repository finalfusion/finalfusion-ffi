include(CTest)

file(COPY testdata DESTINATION ${CMAKE_CURRENT_BINARY_DIR})

add_executable(read_nonexistent_returns_null read_nonexistent_returns_null.c)
target_link_libraries(read_nonexistent_returns_null finalfusion)
add_test(read_nonexistent_returns_null read_nonexistent_returns_null)

add_executable(embedding_lookup embedding_lookup.c)
target_link_libraries(embedding_lookup finalfusion)
add_test(embedding_lookup embedding_lookup)

add_executable(mmap_embeddings mmap_embeddings.c)
target_link_libraries(mmap_embeddings finalfusion)
add_test(mmap_embeddings mmap_embeddings)

add_executable(word2vec_lookup word2vec_lookup.c)
target_link_libraries(word2vec_lookup finalfusion)
add_test(word2vec_lookup word2vec_lookup)

add_executable(text_lookup text_lookup.c)
target_link_libraries(text_lookup finalfusion)
add_test(text_lookup text_lookup)

add_executable(textdims_lookup textdims_lookup.c)
target_link_libraries(textdims_lookup finalfusion)
add_test(textdims_lookup textdims_lookup)

# Ensure that static linking works
add_executable(embedding_lookup_static embedding_lookup.c)
# In the future: collect dynamic library dependencies from Cargo.
target_link_libraries(embedding_lookup_static finalfusion_static dl m pthread)
add_test(embedding_lookup_static embedding_lookup_static)
