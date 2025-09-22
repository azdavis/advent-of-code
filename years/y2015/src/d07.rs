use helpers::HashMap;

type Gate<'a> = &'a str;

#[derive(Debug, Clone, Copy)]
enum Val<'a> {
  Gate(Gate<'a>),
  Num(u32),
}

#[derive(Debug, Clone, Copy)]
enum Expr<'a> {
  Val(Val<'a>),
  Not(Val<'a>),
  And(Val<'a>, Val<'a>),
  Or(Val<'a>, Val<'a>),
  LShift(Val<'a>, Val<'a>),
  RShift(Val<'a>, Val<'a>),
}

fn parse_val(s: &str) -> Val<'_> {
  match s.parse::<u32>() {
    Ok(n) => Val::Num(n),
    Err(_) => Val::Gate(s),
  }
}

fn parse_expr(s: &str) -> Expr<'_> {
  if let Some(s) = s.strip_prefix("NOT ") {
    return Expr::Not(parse_val(s));
  }
  if let Some((lhs, rhs)) = s.split_once(" AND ") {
    return Expr::And(parse_val(lhs), parse_val(rhs));
  }
  if let Some((lhs, rhs)) = s.split_once(" OR ") {
    return Expr::Or(parse_val(lhs), parse_val(rhs));
  }
  if let Some((lhs, rhs)) = s.split_once(" LSHIFT ") {
    return Expr::LShift(parse_val(lhs), parse_val(rhs));
  }
  if let Some((lhs, rhs)) = s.split_once(" RSHIFT ") {
    return Expr::RShift(parse_val(lhs), parse_val(rhs));
  }
  Expr::Val(parse_val(s))
}

type Gates<'a> = HashMap<Gate<'a>, Expr<'a>>;

fn parse(s: &str) -> Gates<'_> {
  s.lines()
    .map(|line| {
      let (expr, gate) = line.split_once(" -> ").unwrap();
      (gate, parse_expr(expr))
    })
    .collect()
}

type Cache<'a> = HashMap<Gate<'a>, u32>;

fn eval_val<'a>(gates: &Gates<'a>, cache: &mut Cache<'a>, val: Val<'a>) -> u32 {
  match val {
    Val::Gate(gate) => {
      if let Some(&n) = cache.get(gate) {
        return n;
      }
      let n = eval_expr(gates, cache, gates[gate]);
      cache.insert(gate, n);
      n
    }
    Val::Num(num) => num,
  }
}

fn eval_op<'a>(
  gates: &Gates<'a>,
  cache: &mut Cache<'a>,
  lhs: Val<'a>,
  rhs: Val<'a>,
  f: fn(u32, u32) -> u32,
) -> u32 {
  f(eval_val(gates, cache, lhs), eval_val(gates, cache, rhs))
}

fn eval_expr<'a>(gates: &Gates<'a>, cache: &mut Cache<'a>, expr: Expr<'a>) -> u32 {
  match expr {
    Expr::Val(val) => eval_val(gates, cache, val),
    Expr::Not(val) => !eval_val(gates, cache, val),
    Expr::And(lhs, rhs) => eval_op(gates, cache, lhs, rhs, |a, b| a & b),
    Expr::Or(lhs, rhs) => eval_op(gates, cache, lhs, rhs, |a, b| a | b),
    Expr::LShift(lhs, rhs) => eval_op(gates, cache, lhs, rhs, |a, b| a << b),
    Expr::RShift(lhs, rhs) => eval_op(gates, cache, lhs, rhs, |a, b| a >> b),
  }
}

pub fn p1(s: &str) -> u32 {
  let gates = parse(s);
  let mut cache = Cache::default();
  eval_val(&gates, &mut cache, Val::Gate("a"))
}

pub fn p2(s: &str) -> u32 {
  let mut gates = parse(s);
  let mut cache = Cache::default();
  let new_gate_b = eval_val(&gates, &mut cache, Val::Gate("a"));
  gates.insert("b", Expr::Val(Val::Num(new_gate_b)));
  cache.clear();
  eval_val(&gates, &mut cache, Val::Gate("a"))
}

#[test]
fn t() {
  let s = include_str!("input/d07.txt");
  assert_eq!(p1(s), 16076);
  assert_eq!(p2(s), 2797);
}
