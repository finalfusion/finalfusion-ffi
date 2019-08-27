if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif()

set(LIB_FILE "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libfinalfusion.so")

file(GLOB_RECURSE LIB_SOURCES "*.rs")

add_custom_command(
    OUTPUT ${LIB_FILE}
    COMMENT "Compiling finalfusion library"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
    DEPENDS ${LIB_SOURCES}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

add_custom_target(finalfusion_target ALL DEPENDS ${LIB_FILE})
add_library(finalfusion SHARED IMPORTED GLOBAL)
add_dependencies(finalfusion finalfusion_target)
set_target_properties(finalfusion PROPERTIES IMPORTED_LOCATION ${LIB_FILE})
