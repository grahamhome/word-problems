use lazy_static::lazy_static;
use regex::Regex;
mod tests;

pub fn answer(command: &str) -> Option<i32> {
    lazy_static! {
        static ref BASIC_PATTERN: Regex = Regex::new(
            // Match the entire string as a math problem
            // Capture the initial value and all operators and operands together
            r"^What\sis\s(-?\d+)((?:\s(?:plus|minus|multiplied\sby|divided\sby|raised\sto\sthe)\s-?\d+(?:(?:nd|rd|th|st])\spower)?)*)\?$"
        )
        .unwrap();
        // Match all operators and operands in the math problem
        // Capture each operator and operand
        static ref OPERATOR_OPERAND_PATTERN: Regex =
            Regex::new(r"(plus|minus|multiplied\sby|divided\sby|raised\sto\sthe)\s(-?\d+)(?:(?:nd|rd|th|st])\spower)?").unwrap();
    }
    let captures = BASIC_PATTERN.captures(command)?;
    let mut result = captures.get(1)?.as_str().parse::<i32>().ok()?;
    for operator_operand_capture in
        OPERATOR_OPERAND_PATTERN.captures_iter(captures.get(2)?.as_str())
    {
        let (operator, operand) = (
            operator_operand_capture.get(1)?.as_str(),
            operator_operand_capture
                .get(2)?
                .as_str()
                .parse::<i32>()
                .ok()?,
        );
        result = match operator {
            "plus" => result + operand,
            "minus" => result - operand,
            "multiplied by" => result * operand,
            "divided by" => result / operand,
            "raised to the" => result.pow(operand.try_into().ok()?),
            _ => return None,
        }
    }
    return Some(result);
}
