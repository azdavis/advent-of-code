pub fn p1(s: &str) -> u64 {
  parse(s).map(eval).sum()
}

pub fn p2(s: &str) -> u64 {
  todo!()
}

#[derive(Debug)]
enum Expr {
  Num(u64),
  BinOp(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug)]
enum BinOp {
  Add,
  Mul,
}

fn eval(e: Expr) -> u64 {
  match e {
    Expr::Num(n) => n,
    Expr::BinOp(lhs, op, rhs) => match op {
      BinOp::Add => eval(*lhs) + eval(*rhs),
      BinOp::Mul => eval(*lhs) * eval(*rhs),
    },
  }
}

fn parse(s: &str) -> impl Iterator<Item = Expr> + '_ {
  s.split('\n')
    .filter(|line| !line.is_empty())
    .map(parse_expr)
}

fn parse_expr(s: &str) -> Expr {
  let mut tokens = tokenize(s);
  tokens.reverse();
  parse_expr_impl(&mut tokens)
}

fn parse_expr_impl(tokens: &mut Vec<Token>) -> Expr {
  let mut ret = parse_expr_atom(tokens);
  loop {
    let op = match tokens.last() {
      Some(&Token::Plus) => BinOp::Add,
      Some(&Token::Star) => BinOp::Mul,
      _ => return ret,
    };
    tokens.pop().unwrap();
    let rhs = parse_expr_atom(tokens);
    ret = Expr::BinOp(ret.into(), op, rhs.into());
  }
}

fn parse_expr_atom(tokens: &mut Vec<Token>) -> Expr {
  match tokens.pop().unwrap() {
    Token::Num(n) => Expr::Num(n),
    Token::LRound => {
      let e = parse_expr_impl(tokens);
      assert_eq!(tokens.pop().unwrap(), Token::RRound);
      e
    }
    tok => panic!("expected number or `(`, found {:?}", tok),
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
  Num(u64),
  LRound,
  RRound,
  Plus,
  Star,
}

const PUNCTUATION: [(u8, Token); 4] = [
  (b'(', Token::LRound),
  (b')', Token::RRound),
  (b'+', Token::Plus),
  (b'*', Token::Star),
];

fn tokenize(s: &str) -> Vec<Token> {
  let mut ret = Vec::new();
  let mut i = 0;
  let bs = s.as_bytes();
  'outer: while let Some(&b) = bs.get(i) {
    if b.is_ascii_whitespace() {
      i += 1;
      continue;
    }
    for &(tok_b, tok) in PUNCTUATION.iter() {
      if b == tok_b {
        ret.push(tok);
        i += 1;
        continue 'outer;
      }
    }
    if b.is_ascii_digit() {
      let start = i;
      i += 1;
      while let Some(&b) = bs.get(i) {
        if b.is_ascii_digit() {
          i += 1
        } else {
          break;
        }
      }
      let s = std::str::from_utf8(&bs[start..i]).unwrap();
      let n: u64 = s.parse().unwrap();
      ret.push(Token::Num(n));
      continue 'outer;
    }
    panic!("invalid byte: {}", b);
  }
  ret
}

#[cfg(test)]
fn e(s: &str) -> u64 {
  eval(parse_expr(s))
}

#[test]
fn t_p1() {
  assert_eq!(e("2 * 3 + (4 * 5)"), 26);
  assert_eq!(e("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
  assert_eq!(e("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
  assert_eq!(e("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
}

#[test]
fn t() {
  let inp = include_str!("input/d18.txt");
  assert_eq!(p1(inp), 3159145843816);
  // assert_eq!(p2(inp), ___);
}
