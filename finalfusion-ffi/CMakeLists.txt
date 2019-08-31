if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif()

file(GLOB_RECURSE LIB_SOURCES "*.rs")

set(SHARED_LIB_FILE "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libfinalfusion.so")
set(STATIC_LIB_FILE "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libfinalfusion.a")

add_custom_command(
    OUTPUT ${SHARED_LIB_FILE} ${STATIC_LIB_FILE}
    COMMENT "Compiling finalfusion library"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
    DEPENDS ${LIB_SOURCES}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

# Dynamic library
add_custom_target(finalfusion_target ALL DEPENDS ${SHARED_LIB_FILE})
add_library(finalfusion SHARED IMPORTED GLOBAL)
add_dependencies(finalfusion finalfusion_target)
set_target_properties(finalfusion PROPERTIES IMPORTED_LOCATION ${SHARED_LIB_FILE})

# Static library
add_custom_target(finalfusion_static_target ALL DEPENDS ${STATIC_LIB_FILE})
add_library(finalfusion_static STATIC IMPORTED GLOBAL)
add_dependencies(finalfusion_static finalfusion_static_target)
set_target_properties(finalfusion_static PROPERTIES IMPORTED_LOCATION ${STATIC_LIB_FILE})
