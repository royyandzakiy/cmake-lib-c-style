#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "complexNumbers::complexNumbers" for configuration "Release"
set_property(TARGET complexNumbers::complexNumbers APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(complexNumbers::complexNumbers PROPERTIES
  IMPORTED_IMPLIB_RELEASE "${_IMPORT_PREFIX}/lib/complexNumbers.lib"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/bin/complexNumbers.dll"
  )

list(APPEND _cmake_import_check_targets complexNumbers::complexNumbers )
list(APPEND _cmake_import_check_files_for_complexNumbers::complexNumbers "${_IMPORT_PREFIX}/lib/complexNumbers.lib" "${_IMPORT_PREFIX}/bin/complexNumbers.dll" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
