# MLNX

## Introduction

MLNX is a experimental linux distribution built with [LLVM](https://llvm.org), [MLIR](https://mlir.llvm.org) and [Nix](https://nixos.org).

## Packages

The current packages available are:

- torch-mlir
- LLVM (including all of the llvm subprojects and runtimes)
- iree

You can build a package such as llvm with `nix build -f ./default.nix packages.llvm`.
Prebuilt packages are not provided, but automated builds are planned.

