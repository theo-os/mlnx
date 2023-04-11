use crate::mlir_sys::{
    mlirExecutionEngineCreate, mlirExecutionEngineDestroy, mlirExecutionEngineInvokePacked,
    MlirExecutionEngine,
};
use crate::{ir::Module, logical_result::LogicalResult, string_ref::StringRef, Error};
use std::ffi::c_void;

/// An execution engine.
pub struct ExecutionEngine {
    raw: MlirExecutionEngine,
}

impl ExecutionEngine {
    /// Creates an ExecutionEngine for the provided Module.
    ///
    /// The Module is expected to be "translatable" to LLVM IR (only contains operations in dialects that implement the LLVMTranslationDialectInterface).
    ///
    /// The module ownership stays with the client and can be destroyed as soon as the call returns.
    ///
    /// optimization_level is the optimization level to be used for transformation and code generation. LLVM passes at optLevel are run before code generation.
    ///
    /// shared_library_paths - The number and array of paths corresponding to shared libraries that will be loaded are specified via numPaths and sharedLibPaths
    pub fn new(
        module: &Module,
        optimization_level: usize,
        shared_library_paths: &[&str],
        enable_obj_dump: bool,
    ) -> Self {
        Self {
            raw: unsafe {
                mlirExecutionEngineCreate(
                    module.to_raw(),
                    optimization_level as i32,
                    shared_library_paths.len() as i32,
                    shared_library_paths
                        .iter()
                        .map(|&string| StringRef::from(string).to_raw())
                        .collect::<Vec<_>>()
                        .as_ptr(),
                    enable_obj_dump,
                )
            },
        }
    }

    /// Invokes a function in a module. The `arguments` argument includes
    /// pointers to results of the function as well as arguments.
    ///
    /// The function must have been tagged with the llvm.emit_c_interface attribute.
    ///
    /// # Safety
    ///
    /// This function modifies memory locations pointed by the `arguments`
    /// argument. If those pointers are invalid or misaligned, calling this
    /// function might result in undefined behavior.
    pub unsafe fn invoke_packed(&self, name: &str, arguments: &mut [*mut ()]) -> Result<(), Error> {
        let result = LogicalResult::from_raw(mlirExecutionEngineInvokePacked(
            self.raw,
            StringRef::from(name).to_raw(),
            arguments.as_mut_ptr() as *mut *mut c_void,
        ));

        if result.is_success() {
            Ok(())
        } else {
            Err(Error::InvokeFunction)
        }
    }
}

impl Drop for ExecutionEngine {
    fn drop(&mut self) {
        unsafe { mlirExecutionEngineDestroy(self.raw) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        context::Context,
        dialect, pass,
        utility::{register_all_dialects, register_all_llvm_translations},
    };

    #[test]
    fn invoke_packed() {
        let registry = dialect::Registry::new();
        register_all_dialects(&registry);

        let context = Context::new();
        context.append_dialect_registry(&registry);
        register_all_llvm_translations(&context);

        let mut module = Module::parse(
            &context,
            r#"
            module {
                func.func @add(%arg0 : i32) -> i32 attributes { llvm.emit_c_interface } {
                    %res = arith.addi %arg0, %arg0 : i32
                    return %res : i32
                }
            }
            "#,
        )
        .unwrap();

        let pass_manager = pass::Manager::new(&context);
        pass_manager.add_pass(pass::conversion::convert_func_to_llvm());

        pass_manager
            .nested_under("func.func")
            .add_pass(pass::conversion::convert_arithmetic_to_llvm());

        assert_eq!(pass_manager.run(&mut module), Ok(()));

        let engine = ExecutionEngine::new(&module, 2, &[], false);

        let mut argument = 42;
        let mut result = -1;

        assert_eq!(
            unsafe {
                engine.invoke_packed(
                    "add",
                    &mut [
                        &mut argument as *mut i32 as *mut (),
                        &mut result as *mut i32 as *mut (),
                    ],
                )
            },
            Ok(())
        );

        assert_eq!(argument, 42);
        assert_eq!(result, 84);
    }
}
