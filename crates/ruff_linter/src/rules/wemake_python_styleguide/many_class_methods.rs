use ruff_python_ast::{self as ast};
use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_text_size::TextRange;
use ruff_python_semantic::analyze::visibility;

use crate::checkers::ast::Checker;


/// Forbid too many methods in a single class.
/// 
/// Reasoning:
///     Having too many methods might lead to the "God object" anti-pattern.
///     This kind of object can handle everything.
///     So, in the end, your code becomes too hard to maintain and test.
/// 
/// Solution:
///     What to do if you have too many methods in a single class?
///     Split this class into several classes,
///     then use composition or inheritance to refactor your code.
///     This will protect you from the "God object" anti-pattern.
/// 
/// We do not make any distinctions between instance and class methods.
/// We also do not care about functions and classes being public or not.
/// We also do not count inherited methods from parents.
/// This rule does not count the attributes of a class.
/// 
/// See also:
///     https://en.wikipedia.org/wiki/God_object
#[violation]
pub struct TooManyMethods {
    methods: usize,
    max_methods: usize,
}


impl Violation for TooManyMethods {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TooManyMethods {
            methods,
            max_methods
        } = self;
        format!("Found too many methods: ({methods} > {max_methods})")
    }
}


pub(crate) fn too_many_methods(checker: &mut Checker, class_def: &ast::StmtClassDef) -> Option<Diagnostic> {
    let mut methods = 0;

    for stmt in class_def.body.iter() {
        if let ast::Stmt::FunctionDef(ast::StmtFunctionDef {decorator_list, ..}) = stmt {
            // Ignore any functions that are `@overload`.
            if visibility::is_overload(decorator_list, checker.semantic()) {
                continue;
            } else {
                methods += 1
            }
        }
    }

    if methods > 2 {
        Some(Diagnostic::new(TooManyMethods { methods, max_methods: 2 }, TextRange::default()))
    } else { None }
}
