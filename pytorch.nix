{
	pkgs ? import <nixpkgs> {},
	pytorchVersion ? "master",
}:
pkgs.stdenv.mkDerivation rec {
	name = "pytorch-${pytorchVersion}";
	version = pytorchVersion;
	src = builtins.fetchTarball {
		url = "https://github.com/pytorch/pytorch/archive/${version}.tar.gz";
	};

	nativeBuildInputs = with pkgs; [
		cmake
		ninja
		python312
	];

	buildInputs = with pkgs; [
		vulkan-headers
		vulkan-loader
		vulkan-validation-layers
		python312
		eigen
	];

	enableParallelBuilding = true;

	cmakeFlags = [
		"-GNinja"
		"-DCMAKE_BUILD_TYPE=MinSizeRel"
		"-DCMAKE_INSTALL_PREFIX=${placeholder "out"}"
		"-DPYTHON_EXECUTABLE=${pkgs.python312}/bin/python"
		"-DPYTHON_INCLUDE_DIR=${pkgs.python312}/include/python3.12"
		"-DPYTHON_LIBRARY=${pkgs.python312}/lib/libpython3.12.so"
		"-DUSE_CUDA=OFF"
		"-DUSE_DISTRIBUTED=OFF"
		"-DUSE_FBGEMM=OFF"
		"-DUSE_KINETO=OFF"
		"-DUSE_MKLDNN=OFF"
		"-DUSE_NNPACK=OFF"
		"-DUSE_QNNPACK=OFF"
		"-DUSE_SYSTEM_EIGEN_INSTALL=ON"
		"-DUSE_SYSTEM_NCCL=OFF"
		"-DUSE_SYSTEM_ONNX=OFF"
		"-DUSE_SYSTEM_ONNX_PROTOBUF=OFF"
		"-DUSE_TENSORPIPE=OFF"
		"-DUSE_VULKAN=ON"
		"-DUSE_XNNPACK=OFF"
	];
}
