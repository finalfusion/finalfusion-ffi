include(CTest)

include_directories ("${PROJECT_SOURCE_DIR}/include")
file(COPY testdata/test.fifu DESTINATION ${CMAKE_CURRENT_BINARY_DIR}/data)

add_executable(read_nonexistent_returns_null read_nonexistent_returns_null.c)
target_link_libraries(read_nonexistent_returns_null finalfusion)
add_test(read_nonexistent_returns_null read_nonexistent_returns_null)

add_executable(embedding_lookup embedding_lookup.c)
target_link_libraries(embedding_lookup finalfusion)
add_test(embedding_lookup embedding_lookup)

add_executable(mmap_embeddings mmap_embeddings.c)
target_link_libraries(mmap_embeddings finalfusion)
add_test(mmap_embeddings mmap_embeddings)

add_executable(cpp_embedding_lookup embedding_lookup.cc)
target_link_libraries(cpp_embedding_lookup embeddings)
add_test(cpp_embedding_lookup cpp_embedding_lookup)
