use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::Decorator;
use ruff_text_size::TextRange;

/// Restrict the maximum number of decorators.
///
/// Reasoning:
///     When you are using too many decorators it means that
///     you are trying to overuse the magic.
///     You have to ask yourself: do I really know what happens inside
///     this decorator tree? Typically, the answer will be "no".
///
/// Solution:
///     Using too many decorators typically means that
///     you are trying to configure the behavior from outside of the class.
///     Do not do that too much.
///     Split functions or classes into smaller ones.
///     Use higher order decorators.
///
/// This rule checks: functions, methods, and classes.
#[violation]
pub struct TooManyDecorators {
    decorators: usize,
    max_decorators: usize,
}

impl Violation for TooManyDecorators {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TooManyDecorators {
            decorators,
            max_decorators,
        } = self;
        format!("Too many decorators: ({decorators} > {max_decorators})")
    }
}

pub(crate) fn too_many_decorators(decorator_list: &[Decorator]) -> Option<Diagnostic> {
    let decorators = decorator_list.len();

    if decorators > 2 {
        Some(Diagnostic::new(
            TooManyDecorators {
                decorators,
                max_decorators: 2,
            },
            TextRange::default(),
        ))
    } else {
        None
    }
}
