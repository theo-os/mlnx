{
	pkgs ? import <nixpkgs> {},
	llvmVersion ? "llvmorg-16.0.0",
	llvmProjects ? "clang;lld;llvm;mlir;clang-tools-extra;libclc;lldb;polly;openmp",
	# TODO: fix "cannot find threads"
	llvmRuntimes ? "",
	llvmTargetTriple ? "x86_64-unknown-linux-gnu",
}:

pkgs.stdenv.mkDerivation rec {
	name = "llvm-full-${version}";
	version = "${llvmVersion}";
	src = builtins.fetchTarball {
		url = "https://github.com/llvm/llvm-project/tarball/${version}";
	};

	enableParallelBuilding = true;

	nativeBuildInputs = with pkgs; [
		cmake
		python312
		perl
		llvmPackages.lld
		gcc12
	];

	cmakeFlags = [
		"-DCMAKE_BUILD_TYPE=MinSizeRel"
		"-DCMAKE_C_COMPILER=gcc"
		"-DCMAKE_CXX_COMPILER=g++"
		"-DLLVM_ENABLE_PROJECTS=${llvmProjects}"
		"-DLLVM_ENABLE_RUNTIMES=${llvmRuntimes}"
		"-DLLVM_ENABLE_LIBXML2=OFF"
		"-DLLVM_ENABLE_TERMINFO=OFF"
		"-DLLVM_ENABLE_ZLIB=OFF"
		"-DLLDB_ENABLE_LUA=OFF"
		"-DLLDB_ENABLE_PYTHON=OFF"
		"-DLLDB_ENABLE_LZMA=OFF"
		"-DLLDB_ENABLE_LIBEDIT=OFF"
		"-DLLDB_ENABLE_LIBXML2=OFF"
		"-DLLDB_ENABLE_CURSES=OFF"
		"-DLLVM_ENABLE_LIBEDIT=OFF"
		"-DLLVM_BUILD_EXAMPLES=OFF"
		"-DLLVM_INCLUDE_EXAMPLES=OFF"
		"-DLLVM_INCLUDE_TESTS=OFF"
		"-DLLVM_INCLUDE_DOCS=OFF"
		"-DLLVM_INCLUDE_BENCHMARKS=OFF"
		"-DLLVM_ENABLE_RTTI=ON"
		"-DLIBCXX_ENABLE_EXCEPTIONS=ON"
		"-DLIBCXXABI_ENABLE_EXCEPTIONS=ON"
		"-DLLVM_ENABLE_EH=ON"
		"-DLLVM_ENABLE_ASSERTIONS=ON"
		"-DLIBCXXABI_ENABLE_THREADS=ON"
		"-DLIBUNWIND_ENABLE_THREADS=ON"
		"-DLIBUNWIND_ENABLE_SHARED=OFF"
		"-DLIBUNWIND_ENABLE_STATIC=ON"
		"-DLIBCXX_ENABLE_SHARED=OFF"
		"-DLIBCXX_ENABLE_STATIC_ABI_LIBRARY=ON"
		"-DLIBCXXABI_ENABLE_SHARED=OFF"
		"-DLIBCXXABI_ENABLE_STATIC_UNWINDER=ON"
		"-DLIBCXXABI_USE_LLVM_UNWINDER=ON"
		"-DLIBCXXABI_USE_COMPILER_RT=ON"
		"-DLLVM_PARALLEL_LINK_JOBS=4"
		"-DCMAKE_INSTALL_PREFIX=${placeholder "out"}"
		"-DLLVM_DEFAULT_TARGET_TRIPLE=${llvmTargetTriple}"
		"-DLIBC_TARGET_TRIPLE=${llvmTargetTriple}"
		"-S"
		"../llvm"
	];
}

