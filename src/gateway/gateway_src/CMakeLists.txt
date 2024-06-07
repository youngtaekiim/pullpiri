#
# Copyright(c) 2022 ZettaScale Technology and others
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License v. 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Eclipse Distribution License
# v. 1.0 which is available at
# http://www.eclipse.org/org/documents/edl-v10.php.
#
# SPDX-License-Identifier: EPL-2.0 OR BSD-3-Clause
#
project(PiccoloGateway LANGUAGES C CXX)
cmake_minimum_required(VERSION 3.8)

SET(CMAKE_PREFIX_PATH ${CMAKE_SOURCE_DIR}/../gateway_thirdlib/)

if(NOT TARGET CycloneDDS-CXX::ddscxx)
  find_package(CycloneDDS-CXX REQUIRED)
endif()

if(NOT TARGET yaml-cpp)
  find_package(yaml-cpp REQUIRED)
endif()

#etcd grpc setup
find_package(Protobuf CONFIG QUIET)
if (NOT Protobuf_FOUND)
    find_package(Protobuf REQUIRED)
endif()

find_package(etcd-cpp-api REQUIRED)
option(BUILD_ETCD_CORE_ONLY "Build etcd-cpp-apiv3 core library (the synchronous runtime) only" OFF)

if(Protobuf_PROTOC_EXECUTABLE)
    if(NOT TARGET protobuf::protoc)
        add_executable(protobuf::protoc IMPORTED)
        if(EXISTS "${Protobuf_PROTOC_EXECUTABLE}")
            set_target_properties(protobuf::protoc PROPERTIES
                IMPORTED_LOCATION "${Protobuf_PROTOC_EXECUTABLE}")
        endif()
    endif()
endif()

find_package(gRPC QUIET)
if(gRPC_FOUND AND TARGET gRPC::grpc)
    # When cross compiling we look for the native grpc_cpp_plugin
    if(CMAKE_CROSSCOMPILING)
        find_program(GRPC_CPP_PLUGIN REQUIRED NAMES grpc_cpp_plugin)
        if(NOT TARGET gRPC::grpc_cpp_plugin)
            add_executable(gRPC::grpc_cpp_plugin IMPORTED)
        endif()
        set_target_properties(gRPC::grpc_cpp_plugin PROPERTIES
                IMPORTED_LOCATION "${GRPC_CPP_PLUGIN}")
    elseif(TARGET gRPC::grpc_cpp_plugin)
        get_target_property(GRPC_CPP_PLUGIN gRPC::grpc_cpp_plugin LOCATION)
    else()
        message(FATAL_ERROR "Found gRPC but no gRPC CPP plugin defined")
    endif()

    set(GRPC_LIBRARIES gRPC::gpr gRPC::grpc gRPC::grpc++)
    get_target_property(GRPC_INCLUDE_DIR gRPC::grpc INTERFACE_INCLUDE_DIRECTORIES)
else()
    include(${CMAKE_CURRENT_SOURCE_DIR}/cmake/FindGRPC.cmake)
    set(GRPC_LIBRARIES ${GPR_LIBRARY} ${GRPC_LIBRARY} ${GRPC_GRPC++_LIBRARY})
endif()
if(gRPC_VERSION VERSION_LESS "1.21" OR gRPC_VERSION VERSION_GREATER "1.31")
    add_definitions(-DWITH_GRPC_CHANNEL_CLASS)
endif()
if(gRPC_VERSION VERSION_LESS "1.17")
    add_definitions(-DWITH_GRPC_CREATE_CHANNEL_INTERNAL_UNIQUE_POINTER)
endif()

get_filename_component(piccologateway_proto "./grpc/PiccoloGateway.proto" ABSOLUTE)
get_filename_component(piccologateway_proto_path "${piccologateway_proto}" PATH)

get_filename_component(statemanager_proto "./grpc/statemanager.proto" ABSOLUTE)
get_filename_component(statemanager_proto_path "${statemanager_proto}" PATH)

get_filename_component(constants_proto "./grpc/constants.proto" ABSOLUTE)
get_filename_component(constants_proto_path "${constants_proto}" PATH)

# Generated sources
set(piccologateway_proto_srcs "${CMAKE_CURRENT_BINARY_DIR}/PiccoloGateway.pb.cc")
set(piccologateway_proto_hdrs "${CMAKE_CURRENT_BINARY_DIR}/PiccoloGateway.pb.h")
set(piccologateway_grpc_srcs "${CMAKE_CURRENT_BINARY_DIR}/PiccoloGateway.grpc.pb.cc")
set(piccologateway_grpc_hdrs "${CMAKE_CURRENT_BINARY_DIR}/PiccoloGateway.grpc.pb.h")
add_custom_command(
      OUTPUT "${piccologateway_proto_srcs}" "${piccologateway_proto_hdrs}" "${piccologateway_grpc_srcs}" "${piccologateway_grpc_hdrs}"
      COMMAND ${Protobuf_PROTOC_EXECUTABLE}
      ARGS --grpc_out "${CMAKE_CURRENT_BINARY_DIR}"
        --cpp_out "${CMAKE_CURRENT_BINARY_DIR}"
        -I "${piccologateway_proto_path}"
        --plugin=protoc-gen-grpc="${GRPC_CPP_PLUGIN}"
        "${piccologateway_proto}"
      DEPENDS "${piccologateway_proto}")

add_library(piccologateway_grpc_proto
  ${piccologateway_grpc_srcs}
  ${piccologateway_grpc_hdrs}
  ${piccologateway_proto_srcs}
  ${piccologateway_proto_hdrs})
target_link_libraries(piccologateway_grpc_proto
  ${GRPC_LIBRARIES}
  )

# Generated sources
set(constants_proto_srcs "${CMAKE_CURRENT_BINARY_DIR}/constants.pb.cc")
set(constants_proto_hdrs "${CMAKE_CURRENT_BINARY_DIR}/constants.pb.h")
set(constants_grpc_srcs "${CMAKE_CURRENT_BINARY_DIR}/constants.grpc.pb.cc")
set(constants_grpc_hdrs "${CMAKE_CURRENT_BINARY_DIR}/constants.grpc.pb.h")
add_custom_command(
      OUTPUT "${constants_proto_srcs}" "${constants_proto_hdrs}" "${constants_grpc_srcs}" "${constants_grpc_hdrs}"
      COMMAND ${Protobuf_PROTOC_EXECUTABLE}
      ARGS --grpc_out "${CMAKE_CURRENT_BINARY_DIR}"
        --cpp_out "${CMAKE_CURRENT_BINARY_DIR}"
        -I "${constants_proto_path}"
        --plugin=protoc-gen-grpc="${GRPC_CPP_PLUGIN}"
        "${constants_proto}"
      DEPENDS "${constants_proto}")

add_library(constants_grpc_proto
  ${constants_grpc_srcs}
  ${constants_grpc_hdrs}
  ${constants_proto_srcs}
  ${constants_proto_hdrs})
target_link_libraries(constants_grpc_proto
  ${GRPC_LIBRARIES}
  )

# Generated sources
set(statemanager_proto_srcs "${CMAKE_CURRENT_BINARY_DIR}/statemanager.pb.cc")
set(statemanager_proto_hdrs "${CMAKE_CURRENT_BINARY_DIR}/statemanager.pb.h")
set(statemanager_grpc_srcs "${CMAKE_CURRENT_BINARY_DIR}/statemanager.grpc.pb.cc")
set(statemanager_grpc_hdrs "${CMAKE_CURRENT_BINARY_DIR}/statemanager.grpc.pb.h")
add_custom_command(
      OUTPUT "${statemanager_proto_srcs}" "${statemanager_proto_hdrs}" "${statemanager_grpc_srcs}" "${statemanager_grpc_hdrs}"
      COMMAND ${Protobuf_PROTOC_EXECUTABLE}
      ARGS --grpc_out "${CMAKE_CURRENT_BINARY_DIR}"
        --cpp_out "${CMAKE_CURRENT_BINARY_DIR}"
        -I "${statemanager_proto_path}"
        --plugin=protoc-gen-grpc="${GRPC_CPP_PLUGIN}"
        "${statemanager_proto}"
      DEPENDS "${statemanager_proto}")

add_library(statemanager_grpc_proto
  ${statemanager_grpc_srcs}
  ${statemanager_grpc_hdrs}
  ${statemanager_proto_srcs}
  ${statemanager_proto_hdrs})
target_link_libraries(statemanager_grpc_proto
  constants_grpc_proto
  ${GRPC_LIBRARIES}
  )

# Include generated *.pb.h files
include_directories("${CMAKE_CURRENT_BINARY_DIR}")

SET(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=gnu++14 -Wall -fpermissive")
SET(SRCS PiccoloGatewayManager.cpp PiccoloGatewayStateManagerCaller.cpp PiccoloGatewayGearListener.cpp PiccoloGatewayGearComparator.cpp PiccoloGatewayParser.cpp PiccoloGatewayServer.cpp PiccoloInfo.cpp main.cpp)
# This is a convenience function, provided by the CycloneDDS package,
# that will supply a library target related the the given idl file.
# In short, it takes the idl file, generates the source files with
# the proper data types and compiles them into a library.

idlcxx_generate(TARGET GearState_lib_cxx FILES gearState.idl WARNINGS no-implicit-extensibility)

# Both executables have only one related source file.
add_executable(PiccoloGateway ${SRCS})
target_compile_options(PiccoloGateway PUBLIC -fpermissive)

# Both executables need to be linked to the idl data type library and
# the ddsc API library.
target_link_libraries(PiccoloGateway
	GearState_lib_cxx
       	CycloneDDS-CXX::ddscxx
	yaml-cpp
	piccologateway_grpc_proto
	statemanager_grpc_proto
	constants_grpc_proto
	absl::flags
	absl::flags_parse
	etcd-cpp-api
)
