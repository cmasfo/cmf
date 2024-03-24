
cmake_minimum_required(VERSION 3.28)

project(cc-winapi)

include_directories(src/inc)

file(GLOB_RECURSE lib_files src/lib/*)

list(LENGTH lib_files lib_files_count)
if(NOT lib_files_count EQUAL 0)
  set(LIB_EXISTS ON)
  set(LIB_NAME ${PROJECT_NAME}_lib)
  add_library(${LIB_NAME})
  target_sources(${LIB_NAME}
  PUBLIC
    FILE_SET CXX_MODULES FILES ${lib_files}
  )
endif()

file(GLOB_RECURSE bin_files src/bin/*)

foreach(file ${bin_files})
  get_filename_component(name ${file} NAME_WLE)
  if(${name} STREQUAL "main")
    set(name ${PROJECT_NAME})
  endif()
  add_executable(${name} ${file})
  if(LIB_EXISTS)
    target_link_libraries(${name} ${LIB_NAME})
  endif()
endforeach()
