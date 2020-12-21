use std::collections::HashSet;

pub fn p1(s: &str) -> usize {
  let recipes = parse(s);
  let ingredients: HashSet<_> = recipes
    .iter()
    .flat_map(|r| r.ingredients.iter())
    .copied()
    .collect();
  let cannot_be_allergen: HashSet<_> = ingredients
    .iter()
    .filter(|&&ing| {
      // the set of possible allergens that a given ingredient might be is the
      // set of allergens definitely present in the recipes that the ingredient
      // is present in.
      //
      // but then, if there exists a recipe that has a certain allergen but does
      // not have the ingredient, then the ingredient cannot be that allergen.
      let possible_allergens: usize = recipes
        .iter()
        .filter(|r| r.ingredients.contains(ing))
        .flat_map(|r| r.allergens.iter())
        .filter(|&&a| {
          recipes
            .iter()
            .all(|r| !r.allergens.contains(a) || r.ingredients.contains(ing))
        })
        .count();
      possible_allergens == 0
    })
    .copied()
    .collect();
  recipes
    .iter()
    .flat_map(|r| r.ingredients.iter())
    .filter(|&&ing| cannot_be_allergen.contains(ing))
    .count()
}

pub fn p2(s: &str) -> u32 {
  todo!()
}

fn parse(s: &str) -> Vec<Recipe<'_>> {
  s.split('\n')
    .filter(|line| !line.is_empty())
    .map(parse_recipe)
    .collect()
}

struct Recipe<'a> {
  ingredients: HashSet<&'a str>,
  allergens: HashSet<&'a str>,
}

fn parse_recipe(s: &str) -> Recipe<'_> {
  let mut parts = s.split(' ');
  let mut ingredients = HashSet::new();
  loop {
    let cur = parts.next().unwrap();
    if cur == "(contains" {
      break;
    }
    assert!(ingredients.insert(cur));
  }
  let allergens: HashSet<_> = parts
    .map(|x| {
      let bs = x.as_bytes();
      let last = *bs.last().unwrap();
      assert!(matches!(last, b',' | b')'));
      std::str::from_utf8(&bs[0..bs.len() - 1]).unwrap()
    })
    .collect();
  Recipe {
    ingredients,
    allergens,
  }
}

#[test]
fn t() {
  let inp = include_str!("input/d21.txt");
  assert_eq!(p1(inp), 1977);
  // assert_eq!(p2(inp), ___);
}
