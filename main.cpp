#include <mlir/IR/Builders.h>
#include <mlir/IR/MLIRContext.h>
#include <mlir/IR/BuiltinOps.h>
#include <nix/eval.hh>
#include <nix/store-api.hh>

int main() {
	mlir::MLIRContext context;
	mlir::OpBuilder builder(&context);
	auto module = builder.create<mlir::ModuleOp>(builder.getUnknownLoc());

	auto func = mlir::FuncOp::create(builder.getUnknownLoc(), "main", builder.getFunctionType({}, {}));

 	auto entryBlock = func.addEntryBlock();
	builder.setInsertionPointToStart(entryBlock);

	auto nix = nix::EvalState(Strings(), nix::openStore());

	module.print(llvm::outs());
	return 0;
}
