#[derive(Debug, Copy, Clone)]
pub enum Token {
  Atom(char),
  Op(char),
  Eof
}
