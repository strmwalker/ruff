use std::collections::HashSet;

use rustc_hash::FxHashSet;

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::{self as ast, Decorator, Expr, ExprAttribute, ExprName, Stmt};
use ruff_text_size::Ranged;

const MAX_ATTRIBUTES: usize = 6;

#[violation]
pub struct TooManyAttributes(usize);

impl Violation for TooManyAttributes {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Found too many attributes: ({} > {})",
            self.0, MAX_ATTRIBUTES
        )
    }
}

#[inline]
fn is_public(x: &str) -> bool {
    return !x.starts_with('_');
}

#[inline]
fn attr_from_assign(expr: &Expr) -> Option<&str> {
    if let Expr::Attribute(ExprAttribute { value, attr, .. }) = expr {
        match **value {
            Expr::Name(ExprName { ref id, .. }) if id == "self" => {
                if is_public(attr.as_str()) {
                    return Some(attr.as_str());
                }
            }
            _ => {}
        }
    }
    None
}

pub(crate) fn too_many_attributes(
    decorator_list: &[Decorator],
    class_def: &ast::StmtClassDef,
) -> Option<Diagnostic> {
    if decorator_list.iter().any(|dec| match dec.expression {
        Expr::Name(ExprName { ref id, .. }) if id == "dataclass" => true,
        _ => false,
    }) {
        return None;
    }

    let mut attrs: HashSet<&str, _> = FxHashSet::default();
    for stmt in &class_def.body {
        match stmt {
            Stmt::Assign(assign) => {
                for target in &assign.targets {
                    if let Expr::Name(ExprName { ref id, .. }) = target {
                        if is_public(id) {
                            attrs.insert(id);
                        }
                    }
                }
            }
            Stmt::AnnAssign(assign) => {
                if let Expr::Name(ExprName { ref id, .. }) = *assign.target {
                    if is_public(id) {
                        attrs.insert(id);
                    }
                }
            }
            Stmt::FunctionDef(func) => {
                for fn_stmt in &func.body {
                    match fn_stmt {
                        Stmt::Assign(assign) => {
                            attrs.extend(assign.targets.iter().filter_map(attr_from_assign))
                        }
                        Stmt::AnnAssign(assign) => {
                            if let Some(attr) = attr_from_assign(assign.target.as_ref()) {
                                attrs.insert(attr);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    if attrs.len() > MAX_ATTRIBUTES {
        Some(Diagnostic::new(
            TooManyAttributes(attrs.len()),
            class_def.name.range(),
        ))
    } else {
        None
    }
}
