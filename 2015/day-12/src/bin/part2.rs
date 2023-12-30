use serde_json::Value;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

fn parse_input(input: &str) -> Value {
    serde_json::from_str(input).expect("valid json")
}

fn sum_numbers(value: &Value) -> i64 {
    match value {
        Value::Number(num) => num
            .as_i64()
            .unwrap_or_else(|| panic!("invalid  i64 number {}", num)),
        Value::Array(values) => values.iter().map(sum_numbers).sum(),
        Value::Object(map) => {
            if map.values().any(|k| k == "red") {
                0
            } else {
                map.values().map(sum_numbers).sum()
            }
        }
        _ => 0,
    }
}

fn get_answer(input: &str) -> i64 {
    let json = parse_input(input);
    sum_numbers(&json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("[1,2,3]", 6; "example 1")]
    #[test_case(r#"{"a":2,"b":4}"#, 6; "example 2")]
    fn example(input: &str, total: i64) {
        assert_eq!(get_answer(input), total);
    }
}
