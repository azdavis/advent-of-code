use helpers::static_regex;
use std::ops::{Add, Mul};

static_regex!(
  RE =
    r"^\w+: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (\d+)$"
);

#[derive(Debug, Clone, Copy)]
struct Ingredient {
  capacity: i32,
  durability: i32,
  flavor: i32,
  texture: i32,
  calories: u32,
}

impl Ingredient {
  fn score(self) -> u32 {
    [self.capacity, self.durability, self.flavor, self.texture]
      .into_iter()
      .map(|it| u32::try_from(it.max(0)).unwrap())
      .product()
  }
}

impl Add for Ingredient {
  type Output = Ingredient;

  fn add(self, rhs: Ingredient) -> Self::Output {
    Ingredient {
      capacity: self.capacity + rhs.capacity,
      durability: self.durability + rhs.durability,
      flavor: self.flavor + rhs.flavor,
      texture: self.texture + rhs.texture,
      calories: self.calories + rhs.calories,
    }
  }
}

impl Mul<u8> for Ingredient {
  type Output = Ingredient;

  fn mul(mut self, rhs: u8) -> Self::Output {
    let scale = i32::from(rhs);
    self.capacity *= scale;
    self.durability *= scale;
    self.flavor *= scale;
    self.texture *= scale;
    self.calories *= u32::from(rhs);
    self
  }
}

fn all_combs(ingredients: &[Ingredient], total: u8) -> Vec<Ingredient> {
  let (&hd, tl) = ingredients.split_first().unwrap();
  if tl.is_empty() {
    return vec![hd * total];
  }
  (0..=total)
    .flat_map(|scale| {
      all_combs(tl, total - scale)
        .into_iter()
        .map(move |ing| ing + (hd * scale))
    })
    .collect()
}

fn run(s: &str, f: fn(&Ingredient) -> bool) -> u32 {
  let ingredients: Vec<_> = s
    .lines()
    .map(|line| {
      let caps = RE.captures(line).unwrap();
      Ingredient {
        capacity: caps[1].parse().unwrap(),
        durability: caps[2].parse().unwrap(),
        flavor: caps[3].parse().unwrap(),
        texture: caps[4].parse().unwrap(),
        calories: caps[5].parse().unwrap(),
      }
    })
    .collect();
  all_combs(&ingredients, 100)
    .into_iter()
    .filter(|&it| f(&it))
    .map(Ingredient::score)
    .max()
    .unwrap()
}

pub fn p1(s: &str) -> u32 {
  run(s, |_| true)
}

pub fn p2(s: &str) -> u32 {
  run(s, |it| it.calories == 500)
}

#[test]
fn t() {
  let s = include_str!("input/d15.txt");
  assert_eq!(p1(s), 222_870);
  assert_eq!(p2(s), 117_936);
}
