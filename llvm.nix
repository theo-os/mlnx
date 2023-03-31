{
	pkgs ? import <nixpkgs> {},
	llvmVersion ? "main",
}:

pkgs.stdenv.mkDerivation rec {
	name = "llvm-full-${version}";
	version = "${llvmVersion}";
	src = builtins.fetchTarball {
		url = "https://github.com/llvm/llvm-project/archive/refs/heads/${version}.tar.gz";
	};

	nativeBuildInputs = with pkgs; [
		cmake
		bmake
		python312
		perl
		llvmPackages.clang
		llvmPackages.lld
	];
	
	configurePhase = ''
	cmake -B build \
	-DCMAKE_MAKE_PROGRAM=bmake \
	-DCMAKE_BUILD_TYPE=MinSizeRel \
	-DCMAKE_C_COMPILER=clang \
	-DCMAKE_CXX_COMPILER=clang++ \
	-DCMAKE_ASM_COMPILER=clang \
	-DLLVM_ENABLE_PROJECTS="all" \
	-DLLVM_ENABLE_RUNTIMES="all" \
	-DLLVM_ENABLE_LIBXML2=OFF \
	-DLLVM_ENABLE_TERMINFO=OFF \
	-DLLVM_ENABLE_ZLIB=OFF \
	-DLLDB_ENABLE_LUA=OFF \
	-DLLVM_ENABLE_LIBCXX=ON \
	-DLLVM_ENABLE_LIBCXXABI=ON \
	-DLLVM_ENABLE_LIBEDIT=OFF \
	-DLLVM_BUILD_EXAMPLES=OFF \
	-DLLVM_INCLUDE_EXAMPLES=OFF \
	-DLLVM_INCLUDE_TESTS=OFF \
	-DLLVM_INCLUDE_DOCS=OFF \
	-DLLVM_INCLUDE_BENCHMARKS=OFF \
	-DLLVM_ENABLE_LLD=ON \
	-DLLVM_ENABLE_RTTI=ON \
	-DLLVM_ENABLE_ASSERTIONS=ON \
	-DLIBCXX_ENABLE_SHARED=OFF \
	-DLIBCXX_ENABLE_STATIC_ABI_LIBRARY=ON \
	-DLIBCXXABI_ENABLE_SHARED=OFF \
	-DLIBCXXABI_ENABLE_STATIC_UNWINDER=ON \
	-DLIBCXXABI_USE_LLVM_UNWINDER=ON \
	-DLIBCXXABI_USE_COMPILER_RT=ON \
	-DLLVM_PARALLEL_LINK_JOBS=2 \
	-DCMAKE_INSTALL_PREFIX=$out \
	./llvm
	'';

	buildPhase = ''
		cmake --build build -- -j $NIX_BUILD_CORES
	'';

	installPhase = ''
		cmake --install build
	'';
}

