use super::TypeLike;
use crate::mlir_sys::{
    mlirFunctionTypeGet, mlirFunctionTypeGetInput, mlirFunctionTypeGetNumInputs,
    mlirFunctionTypeGetNumResults, mlirFunctionTypeGetResult, MlirType,
};
use crate::{ir::Type, utility::into_raw_array, Context, Error};
use std::fmt::{self, Display, Formatter};

/// A function type.
#[derive(Clone, Copy, Debug)]
pub struct Function<'c> {
    r#type: Type<'c>,
}

impl<'c> Function<'c> {
    /// Creates a function type.
    pub fn new(context: &'c Context, inputs: &[Type<'c>], results: &[Type<'c>]) -> Self {
        Self {
            r#type: unsafe {
                Type::from_raw(mlirFunctionTypeGet(
                    context.to_raw(),
                    inputs.len() as isize,
                    into_raw_array(inputs.iter().map(|r#type| r#type.to_raw()).collect()),
                    results.len() as isize,
                    into_raw_array(results.iter().map(|r#type| r#type.to_raw()).collect()),
                ))
            },
        }
    }

    /// Gets an input at a position.
    pub fn input(&self, position: usize) -> Result<Type, Error> {
        if position < self.input_count() {
            unsafe {
                Ok(Type::from_raw(mlirFunctionTypeGetInput(
                    self.r#type.to_raw(),
                    position as isize,
                )))
            }
        } else {
            Err(Error::FunctionInputPosition(self.to_string(), position))
        }
    }

    /// Gets a result at a position.
    pub fn result(&self, position: usize) -> Result<Type, Error> {
        if position < self.result_count() {
            unsafe {
                Ok(Type::from_raw(mlirFunctionTypeGetResult(
                    self.r#type.to_raw(),
                    position as isize,
                )))
            }
        } else {
            Err(Error::FunctionResultPosition(self.to_string(), position))
        }
    }

    /// Gets a number of inputs.
    pub fn input_count(&self) -> usize {
        unsafe { mlirFunctionTypeGetNumInputs(self.r#type.to_raw()) as usize }
    }

    /// Gets a number of results.
    pub fn result_count(&self) -> usize {
        unsafe { mlirFunctionTypeGetNumResults(self.r#type.to_raw()) as usize }
    }
}

impl<'c> TypeLike<'c> for Function<'c> {
    fn to_raw(&self) -> MlirType {
        self.r#type.to_raw()
    }
}

impl<'c> Display for Function<'c> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        Type::from(*self).fmt(formatter)
    }
}

impl<'c> TryFrom<Type<'c>> for Function<'c> {
    type Error = Error;

    fn try_from(r#type: Type<'c>) -> Result<Self, Self::Error> {
        if r#type.is_function() {
            Ok(Self { r#type })
        } else {
            Err(Error::FunctionExpected(r#type.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Context;

    #[test]
    fn new() {
        let context = Context::new();
        let integer = Type::integer(&context, 42);

        assert_eq!(
            Type::from(Function::new(&context, &[integer, integer], &[integer])),
            Type::parse(&context, "(i42, i42) -> i42").unwrap()
        );
    }

    #[test]
    fn multiple_results() {
        let context = Context::new();
        let integer = Type::integer(&context, 42);

        assert_eq!(
            Type::from(Function::new(&context, &[], &[integer, integer])),
            Type::parse(&context, "() -> (i42, i42)").unwrap()
        );
    }

    #[test]
    fn input() {
        let context = Context::new();
        let integer = Type::integer(&context, 42);

        assert_eq!(
            Function::new(&context, &[integer], &[]).input(0),
            Ok(integer)
        );
    }

    #[test]
    fn input_error() {
        let context = Context::new();
        let integer = Type::integer(&context, 42);
        let function = Function::new(&context, &[integer], &[]);

        assert_eq!(
            function.input(42),
            Err(Error::FunctionInputPosition(function.to_string(), 42))
        );
    }

    #[test]
    fn result() {
        let context = Context::new();
        let integer = Type::integer(&context, 42);

        assert_eq!(
            Function::new(&context, &[], &[integer]).result(0),
            Ok(integer)
        );
    }

    #[test]
    fn result_error() {
        let context = Context::new();
        let integer = Type::integer(&context, 42);
        let function = Function::new(&context, &[], &[integer]);

        assert_eq!(
            function.result(42),
            Err(Error::FunctionResultPosition(function.to_string(), 42))
        );
    }

    #[test]
    fn input_count() {
        let context = Context::new();
        let integer = Type::integer(&context, 42);

        assert_eq!(Function::new(&context, &[integer], &[]).input_count(), 1);
    }

    #[test]
    fn result_count() {
        let context = Context::new();
        let integer = Type::integer(&context, 42);

        assert_eq!(Function::new(&context, &[], &[integer]).result_count(), 1);
    }
}
