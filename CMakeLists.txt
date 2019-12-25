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

add_library (jatcher SHARED src/tools.c)
