cmake_minimum_required(VERSION 3.26)
project(mlnx LANGUAGES CXX)

find_package(MLIR REQUIRED CONFIG)
find_package(LLVM REQUIRED CONFIG)
list(APPEND CMAKE_MODULE_PATH ${LLVM_CMAKE_DIR})
list(APPEND CMAKE_MODULE_PATH ${MLIR_CMAKE_DIR})
include(AddLLVM)
include(AddMLIR)
include_directories(${LLVM_INCLUDE_DIRS})
include_directories(${MLIR_INCLUDE_DIRS})
get_property(mlir_libs GLOBAL PROPERTY MLIR_ALL_LIBS)

find_library(nix NAMES nixexpr nixstore nixutil nixcommon)
include_directories(${NIX_INCLUDE_DIRS})

add_executable(mlnx main.cpp)
target_link_libraries(
	mlnx PRIVATE
	${nix}
	${mlir_libs}
)
target_compile_features(mlnx PRIVATE cxx_std_20)
target_compile_options(mlnx PRIVATE -Wall -Wextra -Wpedantic)
target_include_directories(
	mlnx
	PRIVATE
		$<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
		$<INSTALL_INTERFACE:include>
)
