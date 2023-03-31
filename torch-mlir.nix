{
	pkgs ? import <nixpkgs> {},
	torchMlirVersion ? "main",
	llvm,
}:

pkgs.stdenv.mkDerivation rec {
	name = "torch-mlir-${version}";
	version = "${torchMlirVersion}";
	src = builtins.fetchTarball {
		url = "https://github.com/llvm/torch-mlir/archive/${version}.tar.gz";
	};

	nativeBuildInputs = with pkgs; [
		cmake
		bmake
		python312
		perl
	];

	buildInputs = with pkgs; [
		llvm
	];
	
	configurePhase = ''
	cmake -B build \
	-DCMAKE_MAKE_PROGRAM=bmake \
	-DCMAKE_BUILD_TYPE=MinSizeRel \
	-DCMAKE_C_COMPILER=clang \
	-DCMAKE_CXX_COMPILER=clang++ \
	-DCMAKE_ASM_COMPILER=clang \
	-DCMAKE_INSTALL_PREFIX=$out \
	-DMLIR_DIR=${llvm}/lib/cmake/mlir \
	-DLLVM_DIR=${llvm}/lib/cmake/llvm \
	-DTORCH_MLIR_OUT_OF_TREE_BUILD=ON \
	-DMLIR_ENABLE_BINDINGS_PYTHON=OFF
	'';

	buildPhase = ''
		cmake --build build -- -j $NIX_BUILD_CORES
	'';

	installPhase = ''
		cmake --install build
	'';
}
