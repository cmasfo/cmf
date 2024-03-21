
cmake_minimum_required(VERSION 3.28)

project(cmake)

set(CMAKE_C_STANDARD 23)
set(CMAKE_C_EXTENSIONS OFF)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY
${CMAKE_BINARY_DIR}/bin)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_DEBUG
${CMAKE_BINARY_DIR}/bin)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY_RELEASE
${CMAKE_BINARY_DIR}/bin)

function(include_subdirs dir)
  file(GLOB subdirs ${dir}/*)
  foreach(dir ${subdirs})
    if(IS_DIRECTORY ${dir})
      if(EXISTS ${dir}/CMakeLists.txt)
        add_subdirectory(${dir})
      endif()
      include_subdirs(${dir})
    endif()
  endforeach()
endfunction()

include_subdirs(${CMAKE_CURRENT_SOURCE_DIR})
