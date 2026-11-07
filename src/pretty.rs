use crate::term::{Term::Var, Term::Abs, Term::App, *};
use std::fmt;

/// Pretty prints a term.
pub fn pretty_print(term: &Term) -> String {
    match term {
        Var(variable_name) => variable_name.to_string(),
        Abs(param, body) => format!("λ{}. {}", param.to_string(), pretty_print(body)),
        App(term_1, term_2) => format!("{} ({})", pretty_print(term_1), pretty_print(term_2)),
    }
}

/// Display trait implementation for Term.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self))
    }
}