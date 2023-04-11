use crate::mlir_sys::MlirLogicalResult;

/// A logical result of success or failure.
pub struct LogicalResult {
    raw: MlirLogicalResult,
}

// TODO Delete this and replace it with `bool`?
#[allow(unused)]
impl LogicalResult {
    /// Creates a success result.
    pub const fn success() -> Self {
        Self {
            raw: MlirLogicalResult { value: 1 },
        }
    }

    /// Creates a failure result.
    pub const fn failure() -> Self {
        Self {
            raw: MlirLogicalResult { value: 0 },
        }
    }

    /// Returns `true` if a result is success.
    pub const fn is_success(&self) -> bool {
        self.raw.value != 0
    }

    /// Returns `true` if a result is failure.
    pub const fn is_failure(&self) -> bool {
        self.raw.value == 0
    }

    pub(crate) const fn from_raw(result: MlirLogicalResult) -> Self {
        Self { raw: result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        assert!(LogicalResult::success().is_success());
    }

    #[test]
    fn failure() {
        assert!(LogicalResult::failure().is_failure());
    }
}
