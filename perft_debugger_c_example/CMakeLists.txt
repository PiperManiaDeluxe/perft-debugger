cmake_minimum_required(VERSION 3.28.0)

project(perft_debugger_c_example LANGUAGES C)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Variables for building perft-debugger
find_program(CARGO_EXECUTABLE cargo REQUIRED)

set(PERFT_DEBUGGER_CRATE
 "${CMAKE_CURRENT_SOURCE_DIR}/.."
)
set(PERFT_DEBUGGER_TARGET
 "${PERFT_DEBUGGER_CRATE}/target"
)

# Cargo's shared library filename varies by platform
if(WIN32)
    set(PERFT_DEBUGGER_LIBRARY
        "${PERFT_DEBUGGER_TARGET}/release/libperft_debugger.dll"
    )
elseif(APPLE)
    set(PERFT_DEBUGGER_LIBRARY
        "${PERFT_DEBUGGER_TARGET}/release/libperft_debugger.dylib"
    )
else()
    set(PERFT_DEBUGGER_LIBRARY
        "${PERFT_DEBUGGER_TARGET}/release/libperft_debugger.so"
    )
endif()

# Build the perft-debugger library before linking the executable
add_custom_target(build_perft_debugger
    COMMAND
        "${CARGO_EXECUTABLE}" build
        --release
        --manifest-path "${PERFT_DEBUGGER_CRATE}/Cargo.toml"
        --target-dir "${PERFT_DEBUGGER_TARGET}"
    BYPRODUCTS "${PERFT_DEBUGGER_LIBRARY}"
    WORKING_DIRECTORY "${PERFT_DEBUGGER_CRATE}"
    VERBATIM
)

# Link the executable
add_executable(
    perft_debugger_c_example
    main.c
)

target_include_directories(perft_debugger_c_example PRIVATE
    "${PERFT_DEBUGGER_CRATE}/include"
)

set_property(TARGET perft_debugger_c_example PROPERTY C_STANDARD 23)
set_property(TARGET perft_debugger_c_example PROPERTY C_STANDARD_REQUIRED ON)
set_property(TARGET perft_debugger_c_example PROPERTY C_EXTENSIONS OFF)

target_link_directories(perft_debugger_c_example PRIVATE
    "${PERFT_DEBUGGER_TARGET}/release"
)

target_link_libraries(perft_debugger_c_example PRIVATE
    perft_debugger
)

add_dependencies(perft_debugger_c_example build_perft_debugger)

# Help the executable find the shared library at runtime
if (APPLE)
    set_property(TARGET perft_debugger_c_example APPEND PROPERTY BUILD_RPATH
        "@loader_path"
    )
elseif (UNIX)
    set_property(TARGET perft_debugger_c_example APPEND PROPERTY BUILD_RPATH
        "$ORIGIN"
    )
endif()

# Put the shared library next to the executable
add_custom_command(TARGET perft_debugger_c_example POST_BUILD
    COMMAND "${CMAKE_COMMAND}" -E copy_if_different
        "${PERFT_DEBUGGER_LIBRARY}"
        "$<TARGET_FILE_DIR:perft_debugger_c_example>"
)
