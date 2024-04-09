use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{ExceptHandler, StmtTry};

const MAX_EXCEPTS: usize = 3;

/// Forbid too many ``except`` cases in a single ``try`` clause.
///
/// We use :str:`wemake_python_styleguide.constants.MAX_EXCEPT_CASES`
/// as a default value.
///
/// Reasoning:
///     Handling too many exceptions in a single place
///     is a good indicator of a bad design
///     since one controlling structure will become too complex.
///     Also, you will need to test a lot of logic paths in your application.
///
///     If you have too many ``except*`` blocks,
///     it means that the exception group is too complex.
///
/// Solution:
///     We can reduce the complexity of this case by splitting it into multiple
///     ``try`` cases, functions or using a decorator
///     to handle different exceptions.
#[violation]
pub struct TooManyExcepts(usize);

impl Violation for TooManyExcepts {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Too many `except` statements: ({} > {})",
            self.0, MAX_EXCEPTS
        )
    }
}

pub(crate) fn too_many_excepts(stmt: &StmtTry) -> Option<Diagnostic> {
    stmt.handlers
        .iter()
        .skip(MAX_EXCEPTS)
        .take(1)
        .next()
        .map(|handler| {
            Diagnostic::new(TooManyExcepts(stmt.handlers.len()), {
                match handler {
                    ExceptHandler::ExceptHandler(handler) => handler.range,
                }
            })
        })
}
