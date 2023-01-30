use crate::token::Token;
use crate::sexp::Sexp;

#[derive(Debug)]
pub struct Lexer {
  tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut tokens = input
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' |
                'a'..='z' | 'A'..='Z' => Token::Atom(c),
                _ => Token::Op(c),
            })
            .collect::<Vec<_>>();
        tokens.reverse();
        Lexer { tokens }
    }

    pub fn to_sexp(&mut self, min_bp: u8) -> Sexp {
        let mut lhs = match self.next() {
            Token::Atom(t) => Sexp::Atom(t),
            Token::Op(op) => {
                let((), r_bp) = match op {
                    '+' | '-' => ((), 5),
                    _ => panic!("bad op: {:?}", op),
                };
                let rhs = self.to_sexp(r_bp);
                Sexp::Cons(op, vec![rhs])
            }
            t => panic!("bad token {:?}", t),
        };

        loop {
            let op = match self.peek() {
                Token::Eof => break,
                Token::Op(op) => op,
                t => panic!("bad token {:?}", t),
            };

            let (lbp, rbp) = match op {
                '+' | '-' => (1, 2),
                '*' | '/' => (3, 4),
                t => panic!("bad token {:?}", t),
            };

            if lbp < min_bp { break }

            self.next();

            let rhs = self.to_sexp(rbp);

            println!("{:?}", rhs);

            lhs = Sexp::Cons(op, vec![lhs, rhs])
        }

        lhs
    }

    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_a_number() {
        let sexp = Lexer::new("1").to_sexp(0);
        assert_eq!(sexp.to_string(), "1");
    }

    #[test]
    fn should_parse_a_binary_operator() {
        let sexp = Lexer::new("1 + 2 * 3").to_sexp(0);
        assert_eq!(sexp.to_string(), "(+ 1 (* 2 3))");
    }

    #[test]
    fn should_parse_an_unary_minus_on_expression() {
        let sexp = Lexer::new("-1 + 2 * 3").to_sexp(0);
        assert_eq!(sexp.to_string(), "(+ (- 1) (* 2 3))");
    }
}
