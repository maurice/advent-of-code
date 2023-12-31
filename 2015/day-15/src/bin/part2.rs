use std::collections::HashMap;

use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use nom_supreme::parser_ext::ParserExt;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
    assert_eq!(answer, 1766400);
}

#[derive(Debug)]
struct Properties {
    capacity: i8,
    durability: i8,
    flavor: i8,
    texture: i8,
    calories: u8,
}

type Ingredients<'a> = HashMap<&'a str, Properties>;

fn parse_input(input: &str) -> IResult<&str, Ingredients> {
    // Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    static NUM_CHAR: &str = "-0123456789";
    nom::combinator::map(
        all_consuming(separated_list1(
            line_ending,
            tuple((
                alpha1.terminated(tag(": ")),
                // TODO how to create a reusable parser combinator?
                is_a(NUM_CHAR)
                    .preceded_by(tag("capacity "))
                    .terminated(tag(", "))
                    .map_res(|n: &str| n.parse()),
                is_a(NUM_CHAR)
                    .preceded_by(tag("durability "))
                    .terminated(tag(", "))
                    .map_res(|n: &str| n.parse()),
                is_a(NUM_CHAR)
                    .preceded_by(tag("flavor "))
                    .terminated(tag(", "))
                    .map_res(|n: &str| n.parse()),
                is_a(NUM_CHAR)
                    .preceded_by(tag("texture "))
                    .terminated(tag(", "))
                    .map_res(|n: &str| n.parse()),
                is_a(NUM_CHAR)
                    .preceded_by(tag("calories "))
                    .map_res(|n: &str| n.parse()),
            )),
        )),
        |ingredients| {
            ingredients
                .into_iter()
                .map(|(name, capacity, durability, flavor, texture, calories)| {
                    (
                        name,
                        Properties {
                            capacity,
                            durability,
                            flavor,
                            texture,
                            calories,
                        },
                    )
                })
                .collect()
        },
    )(input)
}

fn calculate_properties(recipe: &HashMap<&&str, u8>, ingredients: &Ingredients) -> (i32, u16) {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut calories = 0;
    for (name, properties) in ingredients {
        if let Some(teaspoons) = recipe.get(name) {
            capacity += properties.capacity as i32 * *teaspoons as i32;
            flavor += properties.flavor as i32 * *teaspoons as i32;
            durability += properties.durability as i32 * *teaspoons as i32;
            texture += properties.texture as i32 * *teaspoons as i32;
            calories += properties.calories as u16 * *teaspoons as u16;
        }
    }

    (
        capacity.max(0) * durability.max(0) * flavor.max(0) * texture.max(0),
        calories,
    )
}

fn get_answer(input: &str) -> i32 {
    let ingredients = parse_input(input).unwrap().1;
    let ingredients = dbg!(ingredients);
    let ingredient_names = ingredients.keys().collect::<Vec<_>>();

    let mut max_combo = 0;
    for sprinkles in 0_u8..100 {
        for butterscotch in 0_u8..100 - sprinkles {
            for chocolate in 0_u8..100 - sprinkles - butterscotch {
                let candy = 100 - sprinkles - butterscotch - chocolate;
                assert_eq!(sprinkles + butterscotch + chocolate + candy, 100);
                let mut recipe = HashMap::new();
                recipe.insert(ingredient_names[0], sprinkles);
                recipe.insert(ingredient_names[1], butterscotch);
                recipe.insert(ingredient_names[2], chocolate);
                recipe.insert(ingredient_names[3], candy);
                let (recipe_combo, calories) = calculate_properties(&recipe, &ingredients);
                if calories != 500 {
                    continue;
                }
                max_combo = max_combo.max(recipe_combo);
            }
        }
    }

    max_combo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        let ingredients = parse_input(input).unwrap().1;
        let mut recipe = HashMap::new();
        recipe.insert(&"Butterscotch", 40_u8);
        recipe.insert(&"Cinnamon", 60_u8);
        assert_eq!(calculate_properties(&recipe, &ingredients), (57600000, 500));
    }
}
