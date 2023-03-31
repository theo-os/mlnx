{
	pkgs ? import <nixpkgs> {},
	ireeVersion ? "main",
	llvm,
	torch-mlir,
}:

pkgs.stdenv.mkDerivation rec {
	name = "iree-llvm-${ireeVersion}";
	version = ireeVersion;
	src = builtins.fetchTarball {
		url = "https://github.com/openxla/iree/archive/${ireeVersion}.tar.gz";
	};

	nativeBuildInputs = with pkgs; [
		cmake
		bmake
		python312
		perl
	];

	buildInputs = with pkgs; [
		llvm
		torch-mlir
		vulkan-headers
    vulkan-loader
    vulkan-tools
	];

	configurePhase = ''
	cmake -B build \
	-DCMAKE_MAKE_PROGRAM=bmake \
	-DCMAKE_BUILD_TYPE=MinSizeRel \
	-DCMAKE_INSTALL_PREFIX=$out \
	-DCMAKE_C_COMPILER=${llvm}/bin/clang \
	-DCMAKE_CXX_COMPILER=${llvm}/bin/clang++ \
	-DIREE_BUILD_COMPILER=ON \
	-DIREE_BUILD_TESTS=OFF \
	-DIREE_BUILD_SAMPLES=OFF \
	-DIREE_BUILD_PYTHON_BINDINGS=OFF \
	-DIREE_BUILD_BUNDLED_LLVM=OFF \
	-DIREE_BUILD_BINDINGS_TFLITE_JAVA=OFF \
	-DIREE_HAL_DRIVER_VULKAN=ON \
	-DIREE_TARGET_BACKEND_LLVM_CPU=ON \
	-DIREE_TARGET_BACKEND_LLVM_CPU_WASM=ON \
	-DIREE_TARGET_BACKEND_VULKAN_SPIRV=ON \
	-DIREE_INPUT_TORCH=ON
	'';
	
	buildPhase = ''
		cmake --build build -- -j $NIX_BUILD_CORES
	'';

	installPhase = ''
		cmake --install build
	'';
}
