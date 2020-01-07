cmake_minimum_required(VERSION 3.5.0)
project(Jatcher)

if(NOT DEFINED ENV{JAVA_HOME})
    message(FATAL_ERROR "not defined environment variable:JAVA_HOME")
endif()

if(UNIX)
    include_directories(
            $ENV{JAVA_HOME}/include
            $ENV{JAVA_HOME}/include/linux
    )
elseif(WIN32)
    include_directories(
            $ENV{JAVA_HOME}/include
            $ENV{JAVA_HOME}/include/win32
    )
endif()

execute_process(COMMAND cargo build --out-dir target -Z unstable-options WORKING_DIRECTORY ${PROJECT_SOURCE_DIR})

add_library (jatcher SHARED src/tools.c src/callbacks.c)
target_link_libraries(jatcher ${PROJECT_SOURCE_DIR}/target/libJatcher.a)