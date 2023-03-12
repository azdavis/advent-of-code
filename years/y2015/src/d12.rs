use serde_json::{from_str, Value};

fn parse(s: &str) -> Value {
  from_str(s.trim()).unwrap()
}

fn sum_val(v: &Value, p2: bool) -> i64 {
  match v {
    Value::Null | Value::Bool(_) | Value::String(_) => 0,
    Value::Number(n) => n.as_i64().unwrap(),
    Value::Array(vs) => vs.iter().map(|v| sum_val(v, p2)).sum(),
    Value::Object(obj) => {
      if p2 && obj.values().any(|x| x.as_str() == Some("red")) {
        0
      } else {
        obj.values().map(|v| sum_val(v, p2)).sum()
      }
    }
  }
}

pub fn p1(s: &str) -> i64 {
  sum_val(&parse(s), false)
}

pub fn p2(s: &str) -> i64 {
  sum_val(&parse(s), true)
}

#[test]
fn t() {
  let s = include_str!("input/d12.txt");
  assert_eq!(p1(s), 156_366);
  assert_eq!(p2(s), 96852);
}
