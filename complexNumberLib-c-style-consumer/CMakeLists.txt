cmake_minimum_required(VERSION 3.23)

project(
  complexNumberTest
  VERSION 1.0
  LANGUAGES CXX
)

# Include custom options
include(CMakeOptions.txt)

# Include reusable DLL copy function
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")
include(copy-dlls)  # or copy-imported-dlls

# Find packages will search for a specific library
find_package(complexNumbers REQUIRED)

# Add an executable
add_executable(${CMAKE_PROJECT_NAME} main.cpp)

# Link with the library
target_link_libraries(${CMAKE_PROJECT_NAME} PRIVATE complexNumbers::complexNumbers)

# Include directories
target_include_directories(${CMAKE_PROJECT_NAME} PRIVATE
    ${CMAKE_INSTALL_PREFIX}/include
)

# Set compile definitions (if needed)
if (MSVC)
  if(${ENABLE_SHARED})
    target_compile_definitions(${CMAKE_PROJECT_NAME} PRIVATE COMPILEDLL)
  else()
    target_compile_definitions(${CMAKE_PROJECT_NAME} PRIVATE COMPILELIB)
  endif()
endif()

# Use the reusable function to copy DLLs
copy_dlls_for_target(
    TARGET ${CMAKE_PROJECT_NAME}
    LIBRARY_TARGET complexNumbers::complexNumbers
    DLL_NAME "complexNumbers"  # Optional: defaults to extracted name from target
    DEBUG_POSTFIX "d"          # Optional: defaults to "d"
)