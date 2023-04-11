mod value_like;

pub use self::value_like::ValueLike;
use super::{block, operation, Type};
use crate::mlir_sys::{mlirValueEqual, mlirValuePrint, MlirValue};
use crate::utility::print_callback;
use std::{
    ffi::c_void,
    fmt::{self, Debug, Display, Formatter},
    marker::PhantomData,
};

/// A value.
// Values are always non-owning references to their parents, such as operations
// and block arguments. See the `Value` class in the MLIR C++ API.
#[derive(Clone, Copy)]
pub struct Value<'a> {
    raw: MlirValue,
    _parent: PhantomData<&'a ()>,
}

impl<'a> Value<'a> {
    pub(crate) unsafe fn from_raw(value: MlirValue) -> Self {
        Self {
            raw: value,
            _parent: Default::default(),
        }
    }
}

impl<'a> ValueLike for Value<'a> {
    fn to_raw(&self) -> MlirValue {
        self.raw
    }
}

impl<'a> PartialEq for Value<'a> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { mlirValueEqual(self.raw, other.raw) }
    }
}

impl<'a> Eq for Value<'a> {}

impl<'a> Display for Value<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let mut data = (formatter, Ok(()));

        unsafe {
            mlirValuePrint(
                self.raw,
                Some(print_callback),
                &mut data as *mut _ as *mut c_void,
            );
        }

        data.1
    }
}

impl<'a> Debug for Value<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        writeln!(formatter, "Value(")?;
        Display::fmt(self, formatter)?;
        write!(formatter, ")")
    }
}

impl<'a> From<block::Argument<'a>> for Value<'a> {
    fn from(argument: block::Argument<'a>) -> Self {
        unsafe { Self::from_raw(argument.to_raw()) }
    }
}

impl<'a> From<operation::ResultValue<'a>> for Value<'a> {
    fn from(result: operation::ResultValue<'a>) -> Self {
        unsafe { Self::from_raw(result.to_raw()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        context::Context,
        dialect::{self, Registry},
        ir::{operation, Block, Location, NamedAttribute},
        utility::register_all_dialects,
    };

    #[test]
    fn r#type() {
        let context = Context::new();
        context.set_allow_unregistered_dialects(true);
        let location = Location::unknown(&context);
        let index_type = Type::parse(&context, "index").unwrap();

        let operation = operation::Builder::new("arith.constant", location)
            .add_results(&[index_type])
            .add_attributes(&[NamedAttribute::new_parsed(&context, "value", "0 : index").unwrap()])
            .build();

        assert_eq!(operation.result(0).unwrap().r#type(), index_type);
    }

    #[test]
    fn is_operation_result() {
        let context = Context::new();
        context.set_allow_unregistered_dialects(true);
        let location = Location::unknown(&context);
        let r#type = Type::parse(&context, "index").unwrap();

        let operation = operation::Builder::new("arith.constant", location)
            .add_results(&[r#type])
            .add_attributes(&[NamedAttribute::new_parsed(&context, "value", "0 : index").unwrap()])
            .build();

        assert!(operation.result(0).unwrap().is_operation_result());
    }

    #[test]
    fn is_block_argument() {
        let context = Context::new();
        context.set_allow_unregistered_dialects(true);
        let r#type = Type::parse(&context, "index").unwrap();
        let block = Block::new(&[(r#type, Location::unknown(&context))]);

        assert!(block.argument(0).unwrap().is_block_argument());
    }

    #[test]
    fn dump() {
        let context = Context::new();
        context.set_allow_unregistered_dialects(true);
        let location = Location::unknown(&context);
        let index_type = Type::parse(&context, "index").unwrap();

        let value = operation::Builder::new("arith.constant", location)
            .add_results(&[index_type])
            .add_attributes(&[NamedAttribute::new_parsed(&context, "value", "0 : index").unwrap()])
            .build();

        value.result(0).unwrap().dump();
    }

    #[test]
    fn equal() {
        let context = Context::new();
        let registry = Registry::new();
        register_all_dialects(&registry);
        context.append_dialect_registry(&registry);
        context.load_all_available_dialects();
        let location = Location::unknown(&context);
        let index_type = Type::parse(&context, "index").unwrap();

        let operation = operation::Builder::new("arith.constant", location)
            .add_results(&[index_type])
            .add_attributes(&[NamedAttribute::new_parsed(&context, "value", "0 : index").unwrap()])
            .build();
        let result = Value::from(operation.result(0).unwrap());

        assert_eq!(result, result);
    }

    #[test]
    fn not_equal() {
        let context = Context::new();
        let registry = Registry::new();
        register_all_dialects(&registry);
        context.append_dialect_registry(&registry);
        context.load_all_available_dialects();
        let location = Location::unknown(&context);
        let index_type = Type::parse(&context, "index").unwrap();

        let operation = || {
            operation::Builder::new("arith.constant", location)
                .add_results(&[index_type])
                .add_attributes(&[
                    NamedAttribute::new_parsed(&context, "value", "0 : index").unwrap()
                ])
                .build()
        };

        assert_ne!(
            Value::from(operation().result(0).unwrap()),
            operation().result(0).unwrap().into()
        );
    }

    #[test]
    fn display() {
        let context = Context::new();
        let registry = Registry::new();
        register_all_dialects(&registry);
        context.append_dialect_registry(&registry);
        context.load_all_available_dialects();
        let location = Location::unknown(&context);
        let index_type = Type::parse(&context, "index").unwrap();

        let operation = operation::Builder::new("arith.constant", location)
            .add_results(&[index_type])
            .add_attributes(&[NamedAttribute::new_parsed(&context, "value", "0 : index").unwrap()])
            .build();

        assert_eq!(
            operation.result(0).unwrap().to_string(),
            "%c0 = arith.constant 0 : index\n"
        );
    }

    #[test]
    fn display_with_dialect_loaded() {
        let registry = dialect::Registry::new();
        register_all_dialects(&registry);

        let context = Context::new();
        context.append_dialect_registry(&registry);
        context.load_all_available_dialects();

        let location = Location::unknown(&context);
        let index_type = Type::parse(&context, "index").unwrap();

        let operation = operation::Builder::new("arith.constant", location)
            .add_results(&[index_type])
            .add_attributes(&[NamedAttribute::new_parsed(&context, "value", "0 : index").unwrap()])
            .build();

        assert_eq!(
            operation.result(0).unwrap().to_string(),
            "%c0 = arith.constant 0 : index\n"
        );
    }

    #[test]
    fn debug() {
        let context = Context::new();
        let registry = Registry::new();
        register_all_dialects(&registry);
        context.append_dialect_registry(&registry);
        context.load_all_available_dialects();
        let location = Location::unknown(&context);
        let index_type = Type::parse(&context, "index").unwrap();

        let operation = operation::Builder::new("arith.constant", location)
            .add_results(&[index_type])
            .add_attributes(&[NamedAttribute::new_parsed(&context, "value", "0 : index").unwrap()])
            .build();

        assert_eq!(
            format!("{:?}", Value::from(operation.result(0).unwrap())),
            "Value(\n%c0 = arith.constant 0 : index\n)"
        );
    }
}
