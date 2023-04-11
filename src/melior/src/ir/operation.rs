//! Operations and operation builders.

mod builder;
mod result;

pub use self::{builder::Builder, result::ResultValue};
use super::{BlockRef, Identifier, RegionRef, Value};
use crate::mlir_sys::{
    mlirOpPrintingFlagsCreate, mlirOpPrintingFlagsEnableDebugInfo, mlirOperationClone,
    mlirOperationDestroy, mlirOperationDump, mlirOperationEqual, mlirOperationGetBlock,
    mlirOperationGetContext, mlirOperationGetName, mlirOperationGetNextInBlock,
    mlirOperationGetNumRegions, mlirOperationGetNumResults, mlirOperationGetRegion,
    mlirOperationGetResult, mlirOperationPrintWithFlags, mlirOperationVerify, MlirOperation,
};
use crate::utility::print_debug_callback;
use crate::{
    context::{Context, ContextRef},
    utility::print_callback,
    Error,
};
use core::fmt;
use std::{
    ffi::c_void,
    fmt::{Debug, Display, Formatter},
    marker::PhantomData,
    mem::{forget, transmute},
    ops::Deref,
};

/// An operation.
pub struct Operation<'c> {
    raw: MlirOperation,
    _context: PhantomData<&'c Context>,
}

impl<'c> Operation<'c> {
    /// Gets a context.
    pub fn context(&self) -> ContextRef {
        unsafe { ContextRef::from_raw(mlirOperationGetContext(self.raw)) }
    }

    /// Gets a name.
    pub fn name(&self) -> Identifier {
        unsafe { Identifier::from_raw(mlirOperationGetName(self.raw)) }
    }

    /// Gets a block.
    pub fn block(&self) -> Option<BlockRef> {
        unsafe { BlockRef::from_option_raw(mlirOperationGetBlock(self.raw)) }
    }

    /// Gets a result at a position.
    pub fn result(&self, position: usize) -> Result<result::ResultValue, Error> {
        unsafe {
            if position < self.result_count() {
                Ok(result::ResultValue::from_raw(mlirOperationGetResult(
                    self.raw,
                    position as isize,
                )))
            } else {
                Err(Error::OperationResultPosition(self.to_string(), position))
            }
        }
    }

    /// Gets a number of results.
    pub fn result_count(&self) -> usize {
        unsafe { mlirOperationGetNumResults(self.raw) as usize }
    }

    /// Gets a result at an index.
    pub fn region(&self, index: usize) -> Option<RegionRef> {
        unsafe {
            if index < self.region_count() {
                Some(RegionRef::from_raw(mlirOperationGetRegion(
                    self.raw,
                    index as isize,
                )))
            } else {
                None
            }
        }
    }

    /// Gets a number of regions.
    pub fn region_count(&self) -> usize {
        unsafe { mlirOperationGetNumRegions(self.raw) as usize }
    }

    pub fn debug_print(&self) -> String {
        let mut data = String::new();

        unsafe {
            let flags = mlirOpPrintingFlagsCreate();
            mlirOpPrintingFlagsEnableDebugInfo(flags, true, false);
            mlirOperationPrintWithFlags(
                self.raw,
                flags,
                Some(print_debug_callback),
                &mut data as *mut _ as *mut c_void,
            );
        };

        data
    }

    /// Gets the next operation in the same block.
    pub fn next_in_block(&self) -> Option<OperationRef> {
        unsafe {
            let operation = mlirOperationGetNextInBlock(self.raw);

            if operation.ptr.is_null() {
                None
            } else {
                Some(OperationRef::from_raw(operation))
            }
        }
    }

    /// Verifies an operation.
    pub fn verify(&self) -> bool {
        unsafe { mlirOperationVerify(self.raw) }
    }

    /// Dumps an operation.
    pub fn dump(&self) {
        unsafe { mlirOperationDump(self.raw) }
    }

    pub(crate) unsafe fn from_raw(raw: MlirOperation) -> Self {
        Self {
            raw,
            _context: Default::default(),
        }
    }

    pub(crate) unsafe fn into_raw(self) -> MlirOperation {
        let operation = self.raw;

        forget(self);

        operation
    }
}

impl<'c> Clone for Operation<'c> {
    fn clone(&self) -> Self {
        unsafe { Operation::from_raw(mlirOperationClone(self.raw)) }
    }
}

impl<'c> Drop for Operation<'c> {
    fn drop(&mut self) {
        unsafe { mlirOperationDestroy(self.raw) };
    }
}

impl<'c> PartialEq for Operation<'c> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { mlirOperationEqual(self.raw, other.raw) }
    }
}

impl<'c> Eq for Operation<'c> {}

impl<'a> Display for Operation<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let mut data = (formatter, Ok(()));

        unsafe {
            let flags = mlirOpPrintingFlagsCreate();
            mlirOpPrintingFlagsEnableDebugInfo(flags, false, false);
            mlirOperationPrintWithFlags(
                self.raw,
                flags,
                Some(print_callback),
                &mut data as *mut _ as *mut c_void,
            );
        }

        data.1
    }
}

impl<'c> Debug for Operation<'c> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        writeln!(formatter, "Operation(")?;
        Display::fmt(self, formatter)?;
        write!(formatter, ")")
    }
}

/// A reference to an operation.
// TODO Should we split context lifetimes? Or, is it transitively proven that
// 'c > 'a?
#[derive(Clone, Copy)]
pub struct OperationRef<'a> {
    raw: MlirOperation,
    _reference: PhantomData<&'a Operation<'a>>,
}

impl<'a> OperationRef<'a> {
    pub(crate) const unsafe fn to_raw(self) -> MlirOperation {
        self.raw
    }

    pub(crate) unsafe fn from_raw(raw: MlirOperation) -> Self {
        Self {
            raw,
            _reference: Default::default(),
        }
    }

    pub(crate) unsafe fn from_option_raw(raw: MlirOperation) -> Option<Self> {
        if raw.ptr.is_null() {
            None
        } else {
            Some(Self::from_raw(raw))
        }
    }
}

impl<'a> Deref for OperationRef<'a> {
    type Target = Operation<'a>;

    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self) }
    }
}

impl<'a> PartialEq for OperationRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { mlirOperationEqual(self.raw, other.raw) }
    }
}

impl<'a> Eq for OperationRef<'a> {}

impl<'a> Display for OperationRef<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        Display::fmt(self.deref(), formatter)
    }
}

impl<'a> Debug for OperationRef<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        Debug::fmt(self.deref(), formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        context::Context,
        ir::{Block, Location},
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn new() {
        let context = Context::new();
        context.set_allow_unregistered_dialects(true);
        Builder::new("foo", Location::unknown(&context)).build();
    }

    #[test]
    fn name() {
        let context = Context::new();
        context.set_allow_unregistered_dialects(true);

        assert_eq!(
            Builder::new("foo", Location::unknown(&context))
                .build()
                .name(),
            Identifier::new(&context, "foo")
        );
    }

    #[test]
    fn block() {
        let context = Context::new();
        context.set_allow_unregistered_dialects(true);
        let block = Block::new(&[]);
        let operation =
            block.append_operation(Builder::new("foo", Location::unknown(&context)).build());

        assert_eq!(operation.block().as_deref(), Some(&block));
    }

    #[test]
    fn block_none() {
        let context = Context::new();
        context.set_allow_unregistered_dialects(true);
        assert_eq!(
            Builder::new("foo", Location::unknown(&context))
                .build()
                .block(),
            None
        );
    }

    #[test]
    fn result_error() {
        let context = Context::new();
        context.set_allow_unregistered_dialects(true);
        assert_eq!(
            Builder::new("foo", Location::unknown(&context))
                .build()
                .result(0)
                .unwrap_err(),
            Error::OperationResultPosition("\"foo\"() : () -> ()\n".into(), 0)
        );
    }

    #[test]
    fn region_none() {
        let context = Context::new();
        context.set_allow_unregistered_dialects(true);
        assert!(Builder::new("foo", Location::unknown(&context),)
            .build()
            .region(0)
            .is_none());
    }

    #[test]
    fn clone() {
        let context = Context::new();
        context.set_allow_unregistered_dialects(true);
        let operation = Builder::new("foo", Location::unknown(&context)).build();

        let _ = operation.clone();
    }

    #[test]
    fn display() {
        let context = Context::new();
        context.set_allow_unregistered_dialects(true);

        assert_eq!(
            Builder::new("foo", Location::unknown(&context),)
                .build()
                .to_string(),
            "\"foo\"() : () -> ()\n"
        );
    }

    #[test]
    fn debug() {
        let context = Context::new();
        context.set_allow_unregistered_dialects(true);

        assert_eq!(
            format!(
                "{:?}",
                Builder::new("foo", Location::unknown(&context)).build()
            ),
            "Operation(\n\"foo\"() : () -> ()\n)"
        );
    }

    #[test]
    fn debug_print() {
        let context = Context::new();
        context.set_allow_unregistered_dialects(true);

        let op = Builder::new("foo", Location::new(&context, "file.ext", 1, 1)).build();
        let debug_print = op.debug_print();

        assert_eq!(
            debug_print,
            r#""foo"() : () -> () loc(#loc)
#loc = loc("file.ext":1:1)
"#
        );
    }
}
