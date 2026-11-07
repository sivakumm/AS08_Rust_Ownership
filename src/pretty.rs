use crate::term::{Term::Var, Term::Abs, Term::App, *};
use std::fmt;

/// Pretty prints a term.
pub fn pretty_print(term: &Term) -> String {
    match term {
        Var(str_variable) => str_variable.to_string(),
        Abs(abs_var, abs_term) => format!("λ{}. {}", abs_var.to_string(), pretty_print(abs_term)),
        App(app_term_1, app_term_2) => format!("{} ({})", pretty_print(app_term_1), pretty_print(app_term_2)),
    }
}

/// Display trait implementation for Term.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self))
    }
}