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
in {
	packages = {
		llvm = llvm;
		iree = iree;
		torch-mlir = torch-mlir;
	};
}
