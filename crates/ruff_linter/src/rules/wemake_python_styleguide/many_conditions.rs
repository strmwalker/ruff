use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{Expr, ExprBoolOp};

const MAX_CONDITIONS: usize = 4;

/// Forbid conditions with too many logical operators.
/// 
/// We use :str:`wemake_python_styleguide.constants.MAX_CONDITIONS`
/// as a default value.
/// 
/// Reasoning:
///     When reading through the complex conditions you will fail
///     to understand all the possible branches. And you will end up putting
///     debug breakpoint on this line just to figure out how it works.
/// 
/// Solution:
///     We can reduce the complexity of a single ``if`` by doing two things:
///     creating new variables or creating nested ``if`` statements.
///     Both of these actions will trigger other complexity checks.
/// 
/// We count ``and`` and ``or`` keywords as conditions.
#[violation]
pub struct TooManyConditions(usize);

impl Violation for TooManyConditions {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Found a condition with too much logic:: ({} > {})",
            self.0, MAX_CONDITIONS
        )
    }
}

fn count_conditions(bool_op: &ExprBoolOp) -> usize {
    bool_op
        .values
        .iter()
        .map(|x| match x {
            Expr::BoolOp(nested) => count_conditions(nested),
            _ => 1,
        })
        .sum()
}

pub(crate) fn too_many_conditions(bool_op: &ExprBoolOp) -> Option<Diagnostic> {
    let conditions = count_conditions(bool_op);
    if conditions > MAX_CONDITIONS {
        Some(Diagnostic::new(
            TooManyConditions(conditions),
            bool_op.range,
        ))
    } else {
        None
    }
}
