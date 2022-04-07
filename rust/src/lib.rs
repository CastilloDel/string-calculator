use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum CalculatorError {
    NegativeNumbers(String),
    Parse(ParseIntError),
}

impl From<ParseIntError> for CalculatorError {
    fn from(error: ParseIntError) -> Self {
        CalculatorError::Parse(error)
    }
}

pub fn parse_and_sum(mut input: &str) -> Result<i32, CalculatorError> {
    if input.is_empty() {
        return Ok(0);
    }
    let mut delimiters = vec![",", "\n"];
    if let Some(delimiter) = read_delimiter(input) {
        input = &input[input.find("\n").unwrap() + 1..]; // There must be a \n
        delimiters = vec![delimiter];
    }
    let numbers = split_by_string(input, &delimiters)
        .into_iter()
        .map(|val| val.parse())
        .collect::<Result<Vec<i32>, _>>()?;
    check_negative_numbers(&numbers)?;
    Ok(numbers.into_iter().sum::<i32>())
}

fn read_delimiter(input: &str) -> Option<&str> {
    if input.starts_with("//") {
        return Some(&input[2..input.find('\n')?]);
    }
    None
}

fn check_negative_numbers(numbers: &Vec<i32>) -> Result<(), CalculatorError> {
    let negatives: Vec<i32> = numbers.iter().filter(|&&val| val < 0).copied().collect();
    if negatives.is_empty() {
        Ok(())
    } else {
        let message = negatives
            .into_iter()
            .map(|num| num.to_string())
            .reduce(|s, num| s + ", " + &num)
            .unwrap(); // list can't be empty
        Err(CalculatorError::NegativeNumbers(message))
    }
}

fn split_by_string<'a>(input: &'a str, delimiters: &Vec<&str>) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last_division = 0;
    for i in 1..input.len() {
        let delimiter = delimiters.iter().find(|&&del| input[i..].starts_with(del));
        if let Some(delimiter) = delimiter {
            result.push(&input[last_division..i]);
            last_division = i + delimiter.len();
        }
    }
    result.push(&input[last_division..]);
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_return_0_if_input_is_empty() {
        assert_eq!(parse_and_sum(""), Ok(0));
    }

    #[test]
    fn should_parse_if_there_is_only_one_number() {
        assert_eq!(parse_and_sum("4"), Ok(4));
    }

    #[test]
    fn should_sum_comma_separated_numbers() {
        assert_eq!(parse_and_sum("4,6,2"), Ok(12));
    }

    #[test]
    fn should_sum_newline_separated_numbers() {
        assert_eq!(parse_and_sum("4,6\n3"), Ok(13));
    }

    #[test]
    fn should_be_possible_to_read_a_delimiter() {
        assert_eq!(read_delimiter("//;\n1;4;4"), Some(";"));
        assert_eq!(read_delimiter("//;-\n1;-4;-4"), Some(";-"));
    }

    #[test]
    fn should_be_possible_to_use_a_custom_delimiter() {
        assert_eq!(parse_and_sum("//;-\n1;-4;-4"), Ok(9));
    }

    #[test]
    fn should_error_if_there_are_negative_numbers() {
        assert_eq!(
            parse_and_sum("3,-2,4,-2"),
            Err(CalculatorError::NegativeNumbers("-2, -2".to_string()))
        );
    }
}
