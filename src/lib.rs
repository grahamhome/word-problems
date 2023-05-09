use lazy_static::lazy_static;
use regex::Regex;
mod tests;

pub fn answer(command: &str) -> Option<i32> {
    lazy_static! {
        static ref BASIC_PATTERN: Regex = Regex::new(
            r"^What\sis\s(-?\d+)((?:\s(?:plus|minus|multiplied\sby|divided\sby)\s-?\d+)*)\?$"
        )
        .unwrap();
        static ref OPERATOR_OPERAND_PATTERN: Regex =
            Regex::new(r"(plus|minus|multiplied\sby|divided\sby)\s(-?\d+)").unwrap();
    }
    if let Some(captures) = BASIC_PATTERN.captures(command) {
        if let Ok(mut result) = captures.get(1).unwrap().as_str().parse::<i32>() {
            for operator_operand_capture in
                OPERATOR_OPERAND_PATTERN.captures_iter(captures.get(2).unwrap().as_str())
            {
                let (operator, operand) = (
                    operator_operand_capture.get(1).unwrap().as_str(),
                    operator_operand_capture
                        .get(2)
                        .unwrap()
                        .as_str()
                        .parse::<i32>()
                        .unwrap(),
                );
                match operator {
                    "plus" => result += operand,
                    "minus" => result -= operand,
                    "multiplied by" => result *= operand,
                    "divided by" => result /= operand,
                    _ => panic!(),
                }
            }
            return Some(result);
        } else {
            return None;
        }
    } else {
        return None;
    }
}
