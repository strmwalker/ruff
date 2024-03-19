use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::ExprBoolOp;

const MAX_CONDITIONS: usize = 4;

#[violation]
pub struct TooManyConditions(usize);

impl Violation for TooManyConditions {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Found a condition with too much logic:: ({} > {})", self.0, MAX_CONDITIONS)
    }
}

fn count_conditions(bool_op: &ExprBoolOp) -> usize {
    bool_op.values.iter().map(|x| {
        match x {
            ruff_python_ast::Expr::BoolOp(nested) => count_conditions(nested),
            _ => 1
        }
    }).sum()
}

pub(crate) fn too_many_conditions(bool_op: &ExprBoolOp) -> Option<Diagnostic> {
    let conditions = count_conditions(bool_op);
    if conditions > MAX_CONDITIONS {
        Some(Diagnostic::new(TooManyConditions(conditions), bool_op.range))
    } else {
        None
    }
}
