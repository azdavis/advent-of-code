pub fn p1(s: &str) -> u64 {
  go(s, go_p1)
}

pub fn p2(s: &str) -> u64 {
  go(s, go_p2)
}

fn go<F>(s: &str, f: F) -> u64
where
  F: Fn(&str) -> u64,
{
  s.split('\n').filter(|line| !line.is_empty()).map(f).sum()
}

fn go_p1(s: &str) -> u64 {
  eval(parse_expr(s, prec_p1))
}

fn go_p2(s: &str) -> u64 {
  eval(parse_expr(s, prec_p2))
}

#[derive(Debug)]
enum Expr {
  Num(u64),
  BinOp(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug, Clone, Copy)]
enum BinOp {
  Add,
  Mul,
}

fn prec_p1(b: BinOp) -> usize {
  match b {
    BinOp::Add => 1,
    BinOp::Mul => 1,
  }
}

fn prec_p2(b: BinOp) -> usize {
  match b {
    BinOp::Add => 2,
    BinOp::Mul => 1,
  }
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

fn parse_expr<F>(s: &str, f: F) -> Expr
where
  F: Fn(BinOp) -> usize + Copy,
{
  let mut tokens = tokenize(s);
  tokens.reverse();
  parse_expr_prec(&mut tokens, 0, f)
}

fn parse_expr_prec<F>(tokens: &mut Vec<Token>, min_prec: usize, f: F) -> Expr
where
  F: Fn(BinOp) -> usize + Copy,
{
  let mut ret = parse_expr_atom(tokens, f);
  loop {
    let op = match tokens.last() {
      Some(&Token::Plus) => BinOp::Add,
      Some(&Token::Star) => BinOp::Mul,
      _ => break,
    };
    let prec = f(op);
    if prec < min_prec {
      break;
    }
    tokens.pop().unwrap();
    // the combo of `<` above (not `<=`) and `prec + 1` makes every operator
    // left associative.
    let rhs = parse_expr_prec(tokens, prec + 1, f);
    ret = Expr::BinOp(ret.into(), op, rhs.into());
  }
  ret
}

fn parse_expr_atom<F>(tokens: &mut Vec<Token>, f: F) -> Expr
where
  F: Fn(BinOp) -> usize + Copy,
{
  match tokens.pop().unwrap() {
    Token::Num(n) => Expr::Num(n),
    Token::LRound => {
      let e = parse_expr_prec(tokens, 0, f);
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

#[test]
fn t_p1() {
  assert_eq!(go_p1("2 * 2 + 3"), 7);
  assert_eq!(go_p1("2 * 3 + (4 * 5)"), 26);
  assert_eq!(go_p1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
  assert_eq!(go_p1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
  assert_eq!(
    go_p1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
    13632
  );
}

#[test]
fn t_p2() {
  assert_eq!(go_p2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
  assert_eq!(go_p2("2 * 3 + (4 * 5)"), 46);
  assert_eq!(go_p2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
  assert_eq!(go_p2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
  assert_eq!(
    go_p2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
    23340
  );
}

#[test]
fn t() {
  let inp = include_str!("input/d18.txt");
  assert_eq!(p1(inp), 3159145843816);
  assert_eq!(p2(inp), 55699621957369);
}
