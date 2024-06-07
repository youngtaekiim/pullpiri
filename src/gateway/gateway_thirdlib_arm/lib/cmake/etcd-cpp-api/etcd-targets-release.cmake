#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "etcd-cpp-api" for configuration "Release"
set_property(TARGET etcd-cpp-api APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(etcd-cpp-api PROPERTIES
  IMPORTED_LINK_INTERFACE_LANGUAGES_RELEASE "CXX"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libetcd-cpp-api.a"
  )

list(APPEND _IMPORT_CHECK_TARGETS etcd-cpp-api )
list(APPEND _IMPORT_CHECK_FILES_FOR_etcd-cpp-api "${_IMPORT_PREFIX}/lib/libetcd-cpp-api.a" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
