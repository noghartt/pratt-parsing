use std::fmt;

#[derive(Debug)]
pub enum Sexp {
    Atom(char),
    Cons(char, Vec<Sexp>),
}

impl fmt::Display for Sexp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sexp::Atom(i) => write!(f, "{}", i),
            Sexp::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for sexp in rest {
                    write!(f, " {}", sexp)?;
                }
                write!(f, ")")
            }
        }
    }
}
