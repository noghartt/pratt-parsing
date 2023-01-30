mod lexer;
mod token;
mod sexp;

use lexer::Lexer;

fn main() {
  let input = "1 + 2 * 3";
  let lexer = Lexer::new(input);

  println!("lexer - {:?}", Lexer::new(input).to_sexp(0).to_string());
}
