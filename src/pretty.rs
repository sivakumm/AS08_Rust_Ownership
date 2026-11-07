use crate::term::{Term::Var, Term::Abs, Term::App, *};
use std::fmt;

/// Pretty prints a term.
pub fn pretty_print(term: &Term) -> String {
    match term {
        Var(variable_name) => variable_name.to_string(),
        Abs(param, body) => format!("λ{}. {}", param.to_string(), pretty_print(body)),
        App(term_1, term_2) => {
            let left = match **term_1 {
                Abs(..) => format!("({})", pretty_print(term_1)),
                _ => pretty_print(term_1),
            };
            let right = match **term_2 {
                Var(_) => pretty_print(term_2),
                _ => format!("({})", pretty_print(term_2)),
            };
            format!("{} {}", left, right)
        },
    }
}

/// Display trait implementation for Term.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pretty_var() {
        let result = pretty_print(&var("x"));
        let expected = "x";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pretty_nested_abs() {
        let result = pretty_print(&abs("x", abs("y", var("x"))));
        let expected = "λx. λy. x";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pretty_app_left_assoc() {
        let result = pretty_print(&app(app(var("f"), var("x")), var("y")));
        let expected = "f x y";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pretty_app_right_nested() {
        let result = pretty_print(&app(var("f"), app(var("x"), var("y"))));
        let expected = "f (x y)";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pretty_abs_applied() {
        let result = pretty_print(&app(abs("x", var("x")), abs("y", var("y"))));
        let expected = "(λx. x) (λy. y)";
        assert_eq!(result,expected);
    }
}
