cmake_minimum_required(VERSION 3.10)

project(macroscale_flux VERSION 0.1)

# set cpp standard 
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# needed so that lsp can verify include locations
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# set the target system name
set(CMAKE_SYSTEM_NAME Windows)

# Set the target architecture
set(CMAKE_SYSTEM_PROCESSOR x86_64)

# Specify the cross compiler
set(CMAKE_C_COMPILER /usr/bin/x86_64-w64-mingw32-gcc)
set(CMAKE_CXX_COMPILER /usr/bin/x86_64-w64-mingw32-g++)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -static -static-libgcc -static-libstdc++")

# set output dir
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# include headers 
include_directories(include)

# NvFBC library link
include_directories(vendor/NvFBC/inc)

configure_file(include/config.h.in config.h)
add_executable(capture 
    src/main.cpp
    src/core/event_loop.cpp
    src/core/task_handler.cpp
    src/core/application_data.cpp
    src/core/capturer.cpp
    src/tasks/poll_hotkeys.cpp
)

# includes for binary
target_include_directories(capture PUBLIC "${PROJECT_BINARY_DIR}")
