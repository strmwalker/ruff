use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::StmtIf;

const MAX_ELIFS: usize = 3;

/// Forbid too many ``elif`` branches.
///
/// We use :str:`wemake_python_styleguide.constants.MAX_ELIFS`
/// as a default value.
///
/// Reasoning:
///     This rule is specifically important because many ``elif``
///     branches indicates a complex flow in your design:
///     you are reimplementing ``switch`` in python.
///
/// Solution:
///     There are different design patterns to use instead.
///     For example, you can use an interface that
///     just calls a specific method without ``if``.
///     Another option is to separate your ``if`` into multiple functions.
#[violation]
pub struct TooManyElifs(usize);

impl Violation for TooManyElifs {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Too many `elif` statements: ({} > {})", self.0, MAX_ELIFS)
    }
}

pub(crate) fn too_many_elifs(stmt: &StmtIf) -> Option<Diagnostic> {
    stmt.elif_else_clauses
        .iter()
        .skip(MAX_ELIFS)
        .take(1)
        .next()
        .map(|elif| Diagnostic::new(TooManyElifs(stmt.elif_else_clauses.len()), elif.range))
}
