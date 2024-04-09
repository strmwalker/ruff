use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{self as ast, Stmt, Suite};
use ruff_text_size::TextRange;
use crate::checkers::ast::Checker;
use crate::registry::Rule;

/// bjlhfasglksjdfhglksj
#[violation]
pub struct TooManyImports {
    import_statements: usize,
    max_import_statements: usize,
}

impl Violation for TooManyImports {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TooManyImports {
            import_statements,
            max_import_statements,
        } = self;
        format!("Found module with too many imports: {import_statements} > {max_import_statements}")
    }
}


/// Forbid modules with too many imports.
///
///     Namespaces are one honking great idea -- let's do more of those!
///
///     Reasoning:
///         Having too many imports without prefixes is quite expensive.
///         You have to memorize all the source locations of the imports
///         and sometimes it is hard to remember what kind of functions and classes
///         are already injected into your context.
///
///         It is also a questionable design if a single module has a lot of
///         imports. Why would a single module have so many dependencies?
///         So, the module becomes too coupled.
///
///     Solution:
///         Refactor the imports to import a common namespace. Something like
///         ``from package import module`` and then
///         use it like ``module.function()``.
///
///         Or refactor your code and split the complex module
///         into several modules.
///
///     We do not make any distinction between
///     ``import`` and ``from ... import ...``.
#[violation]
pub struct TooManyImportedNames {
    imported_names: usize,
    max_imported_names: usize,
}

impl Violation for TooManyImportedNames {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TooManyImportedNames {
            imported_names,
            max_imported_names,
        } = self;
        format!("Found module with too many imported names ({imported_names} > {max_imported_names})")
    }
}

fn count_imports(stmts: &[Stmt]) -> (usize, usize) {
    let mut import_statements = 0;
    let mut imported_names = 0;
    for stmt in stmts {
        match stmt {
            Stmt::Import(ast::StmtImport { names, .. }) => {
                import_statements += 1;
                imported_names += names.len()
            }
            Stmt::ImportFrom(ast::StmtImportFrom { names, .. }) => {
                import_statements += 1;
                imported_names += names.len()
            }
            _ => {}
        }
    }
    (import_statements, imported_names)
}

pub(crate) fn module_complexity(checker: &mut Checker, suite: &Suite) {
    let (import_statements, imported_names) = count_imports(suite);
    let range = TextRange::default();
    if checker.enabled(Rule::TooManyImports) {
        if import_statements > 12 {
            checker.diagnostics.push(Diagnostic::new(TooManyImports { import_statements, max_import_statements: 12 }, range));
        }
    }
    if checker.enabled(Rule::TooManyImportedNames) {
        if imported_names > 12 {
            checker.diagnostics.push(Diagnostic::new(TooManyImportedNames { imported_names, max_imported_names: 12 }, range))
        }
    }
}
