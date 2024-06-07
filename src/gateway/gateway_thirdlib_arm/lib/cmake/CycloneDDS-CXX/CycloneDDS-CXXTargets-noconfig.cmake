#----------------------------------------------------------------
# Generated CMake target import file.
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "CycloneDDS-CXX::idlcxx" for configuration ""
set_property(TARGET CycloneDDS-CXX::idlcxx APPEND PROPERTY IMPORTED_CONFIGURATIONS NOCONFIG)
set_target_properties(CycloneDDS-CXX::idlcxx PROPERTIES
  IMPORTED_LOCATION_NOCONFIG "${_IMPORT_PREFIX}/lib/libcycloneddsidlcxx.so.0.11.0"
  IMPORTED_SONAME_NOCONFIG "libcycloneddsidlcxx.so.0"
  )

list(APPEND _IMPORT_CHECK_TARGETS CycloneDDS-CXX::idlcxx )
list(APPEND _IMPORT_CHECK_FILES_FOR_CycloneDDS-CXX::idlcxx "${_IMPORT_PREFIX}/lib/libcycloneddsidlcxx.so.0.11.0" )

# Import target "CycloneDDS-CXX::ddscxx" for configuration ""
set_property(TARGET CycloneDDS-CXX::ddscxx APPEND PROPERTY IMPORTED_CONFIGURATIONS NOCONFIG)
set_target_properties(CycloneDDS-CXX::ddscxx PROPERTIES
  IMPORTED_LOCATION_NOCONFIG "${_IMPORT_PREFIX}/lib/libddscxx.so.0.11.0"
  IMPORTED_SONAME_NOCONFIG "libddscxx.so.0"
  )

list(APPEND _IMPORT_CHECK_TARGETS CycloneDDS-CXX::ddscxx )
list(APPEND _IMPORT_CHECK_FILES_FOR_CycloneDDS-CXX::ddscxx "${_IMPORT_PREFIX}/lib/libddscxx.so.0.11.0" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
