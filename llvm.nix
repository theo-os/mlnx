{
	pkgs ? import <nixpkgs> {},
	llvmVersion ? "llvmorg-16.0.0",
	llvmProjects ? "all",
	llvmRuntimes ? "all",
}:

pkgs.stdenv.mkDerivation rec {
	name = "llvm-full-${version}";
	version = "${llvmVersion}";
	src = builtins.fetchTarball {
		url = "https://github.com/llvm/llvm-project/tarball/${version}";
	};

	nativeBuildInputs = with pkgs; [
		cmake
		ninja
		python312
		perl
		llvmPackages.clang
		llvmPackages.lld
	];
	
	configurePhase = ''
	cmake -B build \
	-G Ninja \
	-DCMAKE_BUILD_TYPE=MinSizeRel \
	-DCMAKE_C_COMPILER=clang \
	-DCMAKE_CXX_COMPILER=clang++ \
	-DCMAKE_ASM_COMPILER=clang \
	-DLLVM_ENABLE_PROJECTS="${llvmProjects}" \
	-DLLVM_ENABLE_RUNTIMES="${llvmRuntimes}" \
	-DLLVM_ENABLE_LIBXML2=OFF \
	-DLLVM_ENABLE_TERMINFO=OFF \
	-DLLVM_ENABLE_ZLIB=OFF \
	-DLLDB_ENABLE_LUA=OFF \
	-DLLDB_ENABLE_PYTHON=OFF \
	-DLLDB_ENABLE_LZMA=OFF \
	-DLLDB_ENABLE_LIBEDIT=OFF \
	-DLLDB_ENABLE_LIBXML2=OFF \
	-DLLDB_ENABLE_CURSES=OFF \
	-DLLVM_ENABLE_LIBEDIT=OFF \
	-DLLVM_BUILD_EXAMPLES=OFF \
	-DLLVM_INCLUDE_EXAMPLES=OFF \
	-DLLVM_INCLUDE_TESTS=OFF \
	-DLLVM_INCLUDE_DOCS=OFF \
	-DLLVM_INCLUDE_BENCHMARKS=OFF \
	-DLLVM_ENABLE_RTTI=ON \
	-DLLVM_ENABLE_ASSERTIONS=ON \
	-DLIBCXX_ENABLE_SHARED=OFF \
	-DLIBCXX_ENABLE_STATIC_ABI_LIBRARY=ON \
	-DLIBCXXABI_ENABLE_SHARED=OFF \
	-DLIBCXXABI_ENABLE_STATIC_UNWINDER=ON \
	-DLIBCXXABI_USE_LLVM_UNWINDER=ON \
	-DLIBCXXABI_USE_COMPILER_RT=ON \
	-DLLVM_PARALLEL_LINK_JOBS=4 \
	-DCMAKE_INSTALL_PREFIX=$out \
	./llvm
	'';

	buildPhase = ''
		cmake --build build --parallel $NIX_BUILD_CORES
	'';

	installPhase = ''
		cmake --install build
	'';
}

