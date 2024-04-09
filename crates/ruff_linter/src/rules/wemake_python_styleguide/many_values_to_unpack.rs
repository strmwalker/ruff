use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{Expr, StmtAssign};

use crate::checkers::ast::Checker;

const MAX_VALUES: usize = 4;

/// Forbid using too many variables to unpack a tuple.
///
/// Reasoning:
///     The order and meaning are hard to remember.
///
/// Solution:
///     If you have more than 2 values in a tuple, consider using
///     ``typing.NamedTuple`` or a dataclass instead.
///
/// Example::
///
///     # Correct:
///     result = foo()
///
///     # Wrong:
///     a, b, c, d, e = foo()
#[violation]
pub struct TooManyValuesToUnpack(usize);

impl Violation for TooManyValuesToUnpack {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Too many variables to unpack a tuple: ({} > {})",
            self.0, MAX_VALUES
        )
    }
}

// AnnAssign doesn't support tuple assignment, so we only care about "just" Assign
pub(crate) fn too_many_values_to_unpack(checker: &mut Checker, assign: &StmtAssign) {
    for target in &assign.targets {
        if let Expr::Tuple(tup) = target {
            // TODO: do we need recursive checking of possible sub-tuples?
            if tup.elts.len() > MAX_VALUES {
                checker.diagnostics.push(Diagnostic::new(
                    TooManyValuesToUnpack(tup.elts.len()),
                    tup.range,
                ))
            }
        }
    }
}
