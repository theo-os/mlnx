let
	pkgs = import (fetchTarball {
		url = "https://github.com/nixos/nixpkgs/archive/nixpkgs-unstable.tar.gz";
	}) {};
	llvm = pkgs.callPackage ./llvm.nix {
		pkgs = pkgs;
	};
	torch-mlir = pkgs.callPackage ./torch-mlir.nix {
		llvm = llvm;
		pkgs = pkgs;
	};
	iree = pkgs.callPackage ./iree.nix {
		llvm = llvm;
		torch-mlir = torch-mlir;
		pkgs = pkgs;
	};
	pytorch = pkgs.callPackage ./pytorch.nix {
		pkgs = pkgs;
		llvm = llvm;
	};
in pkgs.mkShell {
	buildInputs = [
		llvm
		pkgs.cmake pkgs.ninja
	];
	shellHook = ''
		export PATH=${llvm}/bin:$PATH
		export CMAKE_PREFIX_PATH=${llvm}/lib/cmake/mlir:${llvm}/lib/cmake/llvm:$CMAKE_PREFIX_PATH
		export LD_LIBRARY_PATH=${llvm}/lib:${pkgs.nixUnstable}/lib:$LD_LIBRARY_PATH
		export LIBCLANG_PATH=${llvm}/lib
		export MLIR_SYS_PREFIX=${llvm}
	'';
}
