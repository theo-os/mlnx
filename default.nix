let
	pkgs = import <nixpkgs> {};
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
