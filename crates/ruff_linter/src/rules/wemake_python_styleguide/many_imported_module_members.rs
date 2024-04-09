use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{Alias, Stmt};
use ruff_text_size::Ranged;


const MAX_MODULE_MEMBERS: usize = 2;


/// Forbid from ... import ... with too many imported names.
///
/// Reasoning:
///     Importing too many names from one import is an easy way to cause the
///     violation WPS203 - too many imported names.
///
/// Solution:
///     Refactor the imports to import a common namespace.
///     Something like from package import module and then use it like module.function().

#[violation]
pub struct TooManyImportedModuleMembers(usize);


impl Violation for TooManyImportedModuleMembers {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Found too many imported names from a module: ({} > {})", self.0, MAX_MODULE_MEMBERS)
    }
}


pub (crate) fn too_many_imported_module_members(names_module_members: &Vec<Alias>, stmt: &Stmt) -> Option<Diagnostic>{
    let names_module_members_len = names_module_members.len();
    if names_module_members_len > MAX_MODULE_MEMBERS {
        Some(Diagnostic::new(TooManyImportedModuleMembers(names_module_members_len), stmt.range()))
    } else { None }
}
