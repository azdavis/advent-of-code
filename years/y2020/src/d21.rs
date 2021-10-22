use helpers::HashSet;

pub fn p1(s: &str) -> usize {
  let recipes = parse(s);
  let (_, inert) = go(&recipes);
  recipes
    .iter()
    .flat_map(|r| r.ingredients.iter())
    .filter(|&&ing| inert.contains(ing))
    .count()
}

pub fn p2(s: &str) -> String {
  let recipes = parse(s);
  let (mut ingredients, inert) = go(&recipes);
  ingredients.retain(|&ing| !inert.contains(ing));
  let mut allergens: HashSet<_> = recipes
    .iter()
    .flat_map(|r| r.allergens.iter())
    .copied()
    .collect();
  let mut assignment = Vec::new();
  while !ingredients.is_empty() {
    assert_eq!(ingredients.len(), allergens.len());
    let (ing, a) = ingredients
      .iter()
      .find_map(|&ing| {
        let possible_allergens: HashSet<_> = recipes
          .iter()
          .filter(|r| r.ingredients.contains(ing))
          .flat_map(|r| r.allergens.iter())
          .filter(|&&a| {
            allergens.contains(a)
              && recipes.iter().all(|r| {
                !r.allergens.contains(a) || r.ingredients.contains(ing)
              })
          })
          .copied()
          .collect();
        assert!(!possible_allergens.is_empty());
        (possible_allergens.len() == 1).then(|| {
          let mut iter = possible_allergens.into_iter();
          let a = iter.next().unwrap();
          assert!(iter.next().is_none());
          (ing, a)
        })
      })
      .unwrap();
    assert!(ingredients.remove(ing));
    assert!(allergens.remove(a));
    assignment.push((a, ing));
  }
  assignment.sort_unstable();
  let strings: Vec<_> = assignment.into_iter().map(|(_, ing)| ing).collect();
  strings.join(",")
}

fn go<'a>(recipes: &[Recipe<'a>]) -> (HashSet<&'a str>, HashSet<&'a str>) {
  let ingredients: HashSet<_> = recipes
    .iter()
    .flat_map(|r| r.ingredients.iter())
    .copied()
    .collect();
  let inert: HashSet<_> = ingredients
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
  (ingredients, inert)
}

fn parse(s: &str) -> Vec<Recipe<'_>> {
  s.lines().map(parse_recipe).collect()
}

struct Recipe<'a> {
  ingredients: HashSet<&'a str>,
  allergens: HashSet<&'a str>,
}

fn parse_recipe(s: &str) -> Recipe<'_> {
  let mut parts = s.split(' ');
  let mut ingredients = HashSet::default();
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
  let s = include_str!("input/d21.txt");
  assert_eq!(p1(s), 1977);
  assert_eq!(p2(s), "dpkvsdk,xmmpt,cxjqxbt,drbq,zmzq,mnrjrf,kjgl,rkcpxs");
}
