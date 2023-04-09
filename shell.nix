let
	pkgs = import (fetchTarball {
		url = "https://github.com/nixos/nixpkgs/archive/nixpkgs-unstable.tar.gz";
	}) {};
	llvm = pkgs.callPackage ./llvm.nix {};
	torch-mlir = pkgs.callPackage ./torch-mlir.nix {
		llvm = llvm;
	};
	iree = pkgs.callPackage ./iree.nix {
		llvm = llvm;
		torch-mlir = torch-mlir;
	};
in pkgs.mkShell {
	buildInputs = [
		llvm pkgs.cmake pkgs.ninja
		pkgs.llvmPackages_15.clang
		pkgs.llvmPackages_15.libclang
	];
	shellHook = ''
		export PATH=${llvm}/bin:$PATH
		export CMAKE_PREFIX_PATH=${llvm}/lib/cmake/mlir:${llvm}/lib/cmake/llvm:$CMAKE_PREFIX_PATH
		export LD_LIBRARY_PATH=${llvm}/lib:${pkgs.nixUnstable}/lib:$LD_LIBRARY_PATH
	'';
}
